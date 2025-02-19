from functools import singledispatchmethod
import typing

from skema.program_analysis.CAST2FN.cast import CAST

from skema.program_analysis.CAST2FN.model.cast import (
    AstNode,
    Assignment,
    Attribute,
    Call,
    FunctionDef,
    LiteralValue,
    Loop,
    ModelBreak,
    ModelContinue,
    ModelIf,
    ModelReturn,
    Module,
    Name,
    RecordDef,
    ScalarType,
    Var,
)

from skema.program_analysis.CAST2FN.ann_cast.annotated_cast import *
from skema.program_analysis.CAST2FN.model.cast.structure_type import StructureType


class CASTTypeError(TypeError):
    """Used to create errors in the visitor, in particular
    when the visitor encounters some value that it wasn't expecting.

    Args:
        Exception: An exception that occurred during execution.
    """


class CastToAnnotatedCastVisitor:
    """
    class CastToAnnotatedCastVisitor - A visitor that traverses CAST nodes
    and generates an annotated cast version of the CAST.

    The AnnCastNodes have additional attributes (fields) that are used
    in a later pass to maintain scoping information for GrFN containers.
    """

    def __init__(self, cast: CAST):
        self.cast = cast

    def visit_node_list(self, node_list: typing.List[AstNode]):
        return [self.visit(node) for node in node_list]

    def generate_annotated_cast(self, grfn_2_2: bool = False):
        nodes = self.cast.nodes

        annotated_cast = []
        for node in nodes:
            annotated_cast.append(self.visit(node))

        return PipelineState(annotated_cast, grfn_2_2)

    def visit(self, node: AstNode) -> AnnCastNode:
        # print current node being visited.
        # this can be useful for debugging
        # class_name = node.__class__.__name__
        # print(f"\nProcessing node type {class_name}")
        return self._visit(node)

    @singledispatchmethod
    def _visit(self, node: AstNode):
        raise NameError(f"Unrecognized node type: {type(node)}")

    @_visit.register
    def visit_assignment(self, node: Assignment):
        left = self.visit(node.left)
        right = self.visit(node.right)
        return AnnCastAssignment(left, right, node.source_refs)

    @_visit.register
    def visit_attribute(self, node: Attribute):
        value = self.visit(node.value)
        attr = self.visit(node.attr)
        return AnnCastAttribute(value, attr, node.source_refs)

    @_visit.register
    def visit_operator(self, node: Operator):
        operands = self.visit_node_list(node.operands)
        return AnnCastOperator(node.source_language, node.interpreter, node.version, node.op, operands, node.source_refs)
        
    @_visit.register
    def visit_call(self, node: Call):
        func = self.visit(node.func)
        arguments = self.visit_node_list(node.arguments)

        return AnnCastCall(func, arguments, node.source_refs)

    @_visit.register
    def visit_record_def(self, node: RecordDef):
        bases = self.visit_node_list(node.bases)
        funcs = self.visit_node_list(node.funcs)
        fields = self.visit_node_list(node.fields)
        return AnnCastRecordDef(
            node.name, bases, funcs, fields, node.source_refs
        )
        
    @_visit.register
    def visit_function_def(self, node: FunctionDef):
        name = node.name
        args = self.visit_node_list(node.func_args)
        body = self.visit_node_list(node.body)
        return AnnCastFunctionDef(name, args, body, node.source_refs)
        
    @_visit.register
    def visit_literal_value(self, node: LiteralValue):
        if node.value_type == "List[Any]":
            node.value.size = self.visit(
                node.value.size
            )  # Turns the cast var into annCast
            node.value.initial_value = self.visit(
                node.value.initial_value
            )  # Turns the literalValue into annCast
            return AnnCastLiteralValue(
                node.value_type,
                node.value,
                node.source_code_data_type,
                node.source_refs,
            )
        elif node.value_type == StructureType.TUPLE:
            values = self.visit_node_list(node.value)
            return AnnCastLiteralValue(
                node.value_type,
                values,
                node.source_code_data_type,
                node.source_refs,
            )
        return AnnCastLiteralValue(
            node.value_type,
            node.value,
            node.source_code_data_type,
            node.source_refs,
        )

    @_visit.register
    def visit_loop(self, node: Loop):
        if node.pre != None:
            pre = self.visit_node_list(node.pre)
        else:
            pre = []
        if node.post != None and len(node.post) > 0:
            post = self.visit_node_list(node.post)
        else:   
            post = []
        expr = self.visit(node.expr)
        body = self.visit_node_list(node.body)
        return AnnCastLoop(pre, expr, body, post, node.source_refs)

    @_visit.register
    def visit_model_break(self, node: ModelBreak):
        return AnnCastModelBreak(node.source_refs)

    @_visit.register
    def visit_model_continue(self, node: ModelContinue):
        return AnnCastModelContinue(node)

    @_visit.register
    def visit_model_import(self, node: ModelImport):
        return AnnCastModelImport(node)

    @_visit.register
    def visit_model_if(self, node: ModelIf):
        expr = self.visit(node.expr)
        body = self.visit_node_list(node.body)
        orelse = self.visit_node_list(node.orelse)
        return AnnCastModelIf(expr, body, orelse, node.source_refs)

    @_visit.register
    def visit_model_return(self, node: ModelReturn):
        value = self.visit(node.value)
        return AnnCastModelReturn(value, node.source_refs)

    @_visit.register
    def visit_module(self, node: Module):
        body = self.visit_node_list(node.body)
        return AnnCastModule(node.name, body, node.source_refs)

    @_visit.register
    def visit_name(self, node: Name):
        return AnnCastName(node.name, node.id, node.source_refs)

    @_visit.register
    def visit_var(self, node: Var):
        val = self.visit(node.val)
        if node.default_value != None:
            default_value = self.visit(node.default_value)
        else:
            default_value = None
        return AnnCastVar(val, node.type, default_value, node.source_refs)
