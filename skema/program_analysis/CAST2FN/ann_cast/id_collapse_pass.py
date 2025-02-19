from re import A
import typing
from collections import defaultdict
from functools import singledispatchmethod

from skema.program_analysis.CAST2FN.ann_cast.ann_cast_helpers import (
    call_container_name,
)
from skema.program_analysis.CAST2FN.ann_cast.annotated_cast import *
from skema.program_analysis.CAST2FN.model.cast import (
    ScalarType,
    StructureType,
    ValueConstructor,
)


class IdCollapsePass:
    def __init__(self, pipeline_state: PipelineState):
        self.pipeline_state = pipeline_state
        # cache Call nodes so after visiting we can determine which Call's have associated
        # FunctionDefs
        # this dict maps call container name to the AnnCastCall node
        self.cached_call_nodes: typing.Dict[str, AnnCastCall] = {}
        # during the pass, we collpase Name ids to a range starting from zero
        self.old_id_to_collapsed_id = {}
        # this tracks what collapsed ids we have used so far
        self.collapsed_id_counter = 0
        # dict mapping collapsed function id to number of invocations
        # used to populate `invocation_index` of AnnCastCall nodes
        self.func_invocation_counter = defaultdict(int)
        for node in self.pipeline_state.nodes:
            at_module_scope = False
            self.visit(node, at_module_scope)
        self.nodes = self.pipeline_state.nodes
        self.determine_function_defs_for_calls()
        self.store_highest_id()

    def store_highest_id(self):
        self.pipeline_state.collapsed_id_counter = self.collapsed_id_counter

    def collapse_id(self, id: int) -> int:
        """
        Returns the collapsed id for id if it already exists,
        otherwise creates a collapsed id for it
        """
        if id not in self.old_id_to_collapsed_id:
            self.old_id_to_collapsed_id[id] = self.collapsed_id_counter
            self.collapsed_id_counter += 1

        return self.old_id_to_collapsed_id[id]

    def next_function_invocation(self, coll_func_id: int) -> int:
        """
        Returns the next invocation index for function with collapsed id `coll_func_id`
        """
        index = self.func_invocation_counter[coll_func_id]
        self.func_invocation_counter[coll_func_id] += 1

        return index

    def determine_function_defs_for_calls(self):
        for call_name, call in self.cached_call_nodes.items():
            if isinstance(call.func, AnnCastAttribute):
                func_id = call.func.attr.id
            else:
                func_id = call.func.id
            call.has_func_def = self.pipeline_state.func_def_exists(func_id)

            # DEBUG printing
            if self.pipeline_state.PRINT_DEBUGGING_INFO:
                print(f"{call_name} has FunctionDef: {call.has_func_def}")

    def visit(self, node: AnnCastNode, at_module_scope):
        # print current node being visited.
        # this can be useful for debugging
        # class_name = node.__class__.__name__
        # print(f"\nProcessing node type {class_name}")
        try:
            return self._visit(node, at_module_scope)
        except Exception as e:
            print(
                f"id_collapse_pass.py: Error for {type(node)} which has source ref information {node.source_refs}"
            )
            raise e

    def visit_node_list(
        self, node_list: typing.List[AnnCastNode], at_module_scope
    ):
        return [self.visit(node, at_module_scope) for node in node_list]

    @singledispatchmethod
    def _visit(self, node: AnnCastNode, at_module_scope):
        """
        Visit each AnnCastNode, collapsing AnnCastName ids along the way
        """
        print(node.source_refs[0])
        raise Exception(f"Unimplemented AST node of type: {type(node)}")

    @_visit.register
    def visit_assignment(self, node: AnnCastAssignment, at_module_scope):
        self.visit(node.right, at_module_scope)
        # The AnnCastTuple is added to handle scenarios where an assignment
        # is made by assigning to a tuple of values, as opposed to one singular value
        assert (
            isinstance(node.left, AnnCastVar)
            or (isinstance(node.left, AnnCastLiteralValue) and (node.left.value_type == StructureType.TUPLE))
            or isinstance(node.left, AnnCastAttribute)
        ), f"id_collapse: visit_assigment: node.left is {type(node.left)}"
        self.visit(node.left, at_module_scope)

    @_visit.register
    def visit_attribute(self, node: AnnCastAttribute, at_module_scope):
        value = self.visit(node.value, at_module_scope)
        attr = self.visit(node.attr, at_module_scope)

    @_visit.register
    def visit_call(self, node: AnnCastCall, at_module_scope):
        if isinstance(node.func, AnnCastLiteralValue):
            return

        assert isinstance(node.func, AnnCastName) or isinstance(
            node.func, AnnCastAttribute
        ), f"node.func is type f{type(node.func)}"
        if isinstance(node.func, AnnCastName):
            node.func.id = self.collapse_id(node.func.id)
            node.invocation_index = self.next_function_invocation(node.func.id)
        else:
            if isinstance(node.func.value, AnnCastCall):
                self.visit(node.func.value, at_module_scope)
            elif isinstance(node.func.value, AnnCastAttribute):
                self.visit(node.func.value, at_module_scope)
            #elif isinstance(node.func.value, AnnCastSubscript):
            #    self.visit(node.func.value, at_module_scope)
            elif isinstance(node.func.value, AnnCastOperator):
                self.visit(node.func.value, at_module_scope)
            elif isinstance(node.func.value, AnnCastAssignment):
                self.visit(node.func.value, at_module_scope)
            else:
                if not isinstance(node.func.value, AnnCastLiteralValue):
                    node.func.value.id = self.collapse_id(node.func.value.id)
            node.func.attr.id = self.collapse_id(node.func.attr.id)
            node.invocation_index = self.next_function_invocation(
                node.func.attr.id
            )

        # cache Call node to later determine if this Call has a FunctionDef
        call_name = call_container_name(node)
        self.cached_call_nodes[call_name] = node

        self.visit_node_list(node.arguments, at_module_scope)

    @_visit.register
    def visit_record_def(self, node: AnnCastRecordDef, at_module_scope):
        at_module_scope = False

        # Each base should be an AnnCastName node
        self.visit_node_list(node.bases, at_module_scope)

        # Each func is an AnnCastFuncDef node
        self.visit_node_list(node.funcs, at_module_scope)

        # Each field (attribute) is an AnnCastVar node
        self.visit_node_list(node.fields, at_module_scope)

    @_visit.register
    def visit_function_def(self, node: AnnCastFunctionDef, at_module_scope):
        # collapse the function id
        node.name.id = self.collapse_id(node.name.id)
        self.pipeline_state.func_id_to_def[node.name.id] = node

        at_module_scope = False
        self.visit_node_list(node.func_args, at_module_scope)
        self.visit_node_list(node.body, at_module_scope)

    @_visit.register
    def visit_literal_value(self, node: AnnCastLiteralValue, at_module_scope):
        if node.value_type == "List[Any]":
            # operator - string
            # size - Var node or a LiteralValue node (for number)
            # initial_value - LiteralValue node
            val = node.value
            self.visit(val.size, at_module_scope)

            # List literal doesn't need to add any other changes
            # to the anncast at this pass

        elif node.value_type == StructureType.TUPLE: # or node.value_type == StructureType.LIST:
            self.visit_node_list(node.value, at_module_scope)
        elif node.value_type == ScalarType.INTEGER:
            pass
        elif node.value_type == ScalarType.ABSTRACTFLOAT:
            pass
        pass

    @_visit.register
    def visit_loop(self, node: AnnCastLoop, at_module_scope):
        self.visit_node_list(node.pre, at_module_scope)
        self.visit(node.expr, at_module_scope)
        self.visit_node_list(node.body, at_module_scope)
        self.visit_node_list(node.post, at_module_scope)

    @_visit.register
    def visit_model_break(self, node: AnnCastModelBreak, at_module_scope):
        pass

    @_visit.register
    def visit_model_continue(
        self, node: AnnCastModelContinue, at_module_scope
    ):
        pass

    @_visit.register
    def visit_model_if(self, node: AnnCastModelIf, at_module_scope):
        self.visit(node.expr, at_module_scope)
        self.visit_node_list(node.body, at_module_scope)
        self.visit_node_list(node.orelse, at_module_scope)

    @_visit.register
    def visit_return(self, node: AnnCastModelReturn, at_module_scope):
        self.visit(node.value, at_module_scope)

    @_visit.register
    def visit_model_import(self, node: AnnCastModelImport, at_module_scope):
        pass

    @_visit.register
    def visit_module(self, node: AnnCastModule, at_module_scope):
        # we cache the module node in the AnnCast object
        self.pipeline_state.module_node = node
        at_module_scope = True
        self.visit_node_list(node.body, at_module_scope)

    @_visit.register
    def visit_name(self, node: AnnCastName, at_module_scope):
        node.id = self.collapse_id(node.id)

        # we consider name nodes at the module scope to be globals
        # and store them in the `used_vars` attribute of the module_node
        if at_module_scope:
            self.pipeline_state.module_node.used_vars[node.id] = node.name

    @_visit.register
    def visit_operator(self, node: AnnCastOperator, at_module_scope):
        # visit operands
        self.visit_node_list(node.operands, at_module_scope)

    @_visit.register
    def visit_var(self, node: AnnCastVar, at_module_scope):
        self.visit(node.val, at_module_scope)
        if node.default_value != None:
            self.visit(node.default_value, at_module_scope)

    @_visit.register
    def visit_tuple(self, node: AnnCastTuple, at_module_scope):
        # Tuple of vars: Visit them all to collapse IDs, nothing else to be done I think
        self.visit_node_list(node.values, at_module_scope)
