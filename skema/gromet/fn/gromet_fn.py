# coding: utf-8

"""
    Grounded Model Exchange (GroMEt) schema for Function Networks

    This document defines the GroMEt Function Network data model. Note that Metadata is defined in separate spec.  __Using Swagger to Generate Class Structure__  To automatically generate Python or Java models corresponding to this document, you can use [swagger-codegen](https://swagger.io/tools/swagger-codegen/). This can be used to generate the client code based off of this spec, and in the process this will generate the data model class structure.  1. Install via the method described for your operating system    [here](https://github.com/swagger-api/swagger-codegen#Prerequisites).    Make sure to install a version after 3.0 that will support openapi 3. 2. Run swagger-codegen with the options in the example below.    The URL references where the yaml for this documentation is stored on    github. Make sure to replace CURRENT_VERSION with the correct version.    (The current version is `0.1.4`.)    To generate Java classes rather, change the `-l python` to `-l java`.    Change the value to the `-o` option to the desired output location.    ```    swagger-codegen generate -l python -o ./client -i https://raw.githubusercontent.com/ml4ai/automates-v2/master/docs/source/gromet_FN_v{CURRENT_VERSION}.yaml    ``` 3. Once it executes, the client code will be generated at your specified    location.    For python, the classes will be located in    `$OUTPUT_PATH/swagger_client/models/`.    For java, they will be located in    `$OUTPUT_PATH/src/main/java/io/swagger/client/model/`  If generating GroMEt schema data model classes in SKEMA (AutoMATES), then after generating the above, follow the instructions here: ``` <automates>/automates/model_assembly/gromet/model/README.md ```   # noqa: E501

    OpenAPI spec version: 0.1.6
    Contact: claytonm@arizona.edu
    Generated by: https://github.com/swagger-api/swagger-codegen.git
"""

import pprint
import re  # noqa: F401

import six
from skema.gromet.fn.gromet_object import GrometObject  # noqa: F401,E501

class GrometFN(GrometObject):
    """NOTE: This class is auto generated by the swagger code generator program.

    Do not edit the class manually.
    """
    """
    Attributes:
      swagger_types (dict): The key is attribute name
                            and the value is attribute type.
      attribute_map (dict): The key is attribute name
                            and the value is json key in definition.
    """
    swagger_types = {
        'name': 'str',
        'b': 'list[GrometBoxFunction]',
        'opi': 'list[GrometPort]',
        'opo': 'list[GrometPort]',
        'wopio': 'list[GrometWire]',
        'bf': 'list[GrometBoxFunction]',
        'pif': 'list[GrometPort]',
        'pof': 'list[GrometPort]',
        'wfopi': 'list[GrometWire]',
        'wfl': 'list[GrometWire]',
        'wff': 'list[GrometWire]',
        'wfc': 'list[GrometWire]',
        'wfopo': 'list[GrometWire]',
        'bl': 'list[GrometBoxLoop]',
        'pil': 'list[GrometPort]',
        'pol': 'list[GrometPort]',
        'wlopi': 'list[GrometWire]',
        'wll': 'list[GrometWire]',
        'wlf': 'list[GrometWire]',
        'wlc': 'list[GrometWire]',
        'wlopo': 'list[GrometWire]',
        'bc': 'list[GrometBoxConditional]',
        'pic': 'list[GrometPort]',
        'poc': 'list[GrometPort]',
        'wcopi': 'list[GrometWire]',
        'wcl': 'list[GrometWire]',
        'wcf': 'list[GrometWire]',
        'wcc': 'list[GrometWire]',
        'wcopo': 'list[GrometWire]'
    }
    if hasattr(GrometObject, "swagger_types"):
        swagger_types.update(GrometObject.swagger_types)

    attribute_map = {
        'name': 'name',
        'b': 'b',
        'opi': 'opi',
        'opo': 'opo',
        'wopio': 'wopio',
        'bf': 'bf',
        'pif': 'pif',
        'pof': 'pof',
        'wfopi': 'wfopi',
        'wfl': 'wfl',
        'wff': 'wff',
        'wfc': 'wfc',
        'wfopo': 'wfopo',
        'bl': 'bl',
        'pil': 'pil',
        'pol': 'pol',
        'wlopi': 'wlopi',
        'wll': 'wll',
        'wlf': 'wlf',
        'wlc': 'wlc',
        'wlopo': 'wlopo',
        'bc': 'bc',
        'pic': 'pic',
        'poc': 'poc',
        'wcopi': 'wcopi',
        'wcl': 'wcl',
        'wcf': 'wcf',
        'wcc': 'wcc',
        'wcopo': 'wcopo'
    }
    if hasattr(GrometObject, "attribute_map"):
        attribute_map.update(GrometObject.attribute_map)

    def __init__(self, name=None, b=None, opi=None, opo=None, wopio=None, bf=None, pif=None, pof=None, wfopi=None, wfl=None, wff=None, wfc=None, wfopo=None, bl=None, pil=None, pol=None, wlopi=None, wll=None, wlf=None, wlc=None, wlopo=None, bc=None, pic=None, poc=None, wcopi=None, wcl=None, wcf=None, wcc=None, wcopo=None, *args, **kwargs):  # noqa: E501
        """GrometFN - a model defined in Swagger"""  # noqa: E501
        self._name = None
        self._b = None
        self._opi = None
        self._opo = None
        self._wopio = None
        self._bf = None
        self._pif = None
        self._pof = None
        self._wfopi = None
        self._wfl = None
        self._wff = None
        self._wfc = None
        self._wfopo = None
        self._bl = None
        self._pil = None
        self._pol = None
        self._wlopi = None
        self._wll = None
        self._wlf = None
        self._wlc = None
        self._wlopo = None
        self._bc = None
        self._pic = None
        self._poc = None
        self._wcopi = None
        self._wcl = None
        self._wcf = None
        self._wcc = None
        self._wcopo = None
        self.discriminator = None
        if name is not None:
            self.name = name
        if b is not None:
            self.b = b
        if opi is not None:
            self.opi = opi
        if opo is not None:
            self.opo = opo
        if wopio is not None:
            self.wopio = wopio
        if bf is not None:
            self.bf = bf
        if pif is not None:
            self.pif = pif
        if pof is not None:
            self.pof = pof
        if wfopi is not None:
            self.wfopi = wfopi
        if wfl is not None:
            self.wfl = wfl
        if wff is not None:
            self.wff = wff
        if wfc is not None:
            self.wfc = wfc
        if wfopo is not None:
            self.wfopo = wfopo
        if bl is not None:
            self.bl = bl
        if pil is not None:
            self.pil = pil
        if pol is not None:
            self.pol = pol
        if wlopi is not None:
            self.wlopi = wlopi
        if wll is not None:
            self.wll = wll
        if wlf is not None:
            self.wlf = wlf
        if wlc is not None:
            self.wlc = wlc
        if wlopo is not None:
            self.wlopo = wlopo
        if bc is not None:
            self.bc = bc
        if pic is not None:
            self.pic = pic
        if poc is not None:
            self.poc = poc
        if wcopi is not None:
            self.wcopi = wcopi
        if wcl is not None:
            self.wcl = wcl
        if wcf is not None:
            self.wcf = wcf
        if wcc is not None:
            self.wcc = wcc
        if wcopo is not None:
            self.wcopo = wcopo
        GrometObject.__init__(self, *args, **kwargs)

    @property
    def name(self):
        """Gets the name of this GrometFN.  # noqa: E501

        The name of the Function Network   # noqa: E501

        :return: The name of this GrometFN.  # noqa: E501
        :rtype: str
        """
        return self._name

    @name.setter
    def name(self, name):
        """Sets the name of this GrometFN.

        The name of the Function Network   # noqa: E501

        :param name: The name of this GrometFN.  # noqa: E501
        :type: str
        """

        self._name = name

    @property
    def b(self):
        """Gets the b of this GrometFN.  # noqa: E501

        b: The FN Outer Box (although not enforced, there is always only 1).   # noqa: E501

        :return: The b of this GrometFN.  # noqa: E501
        :rtype: list[GrometBoxFunction]
        """
        return self._b

    @b.setter
    def b(self, b):
        """Sets the b of this GrometFN.

        b: The FN Outer Box (although not enforced, there is always only 1).   # noqa: E501

        :param b: The b of this GrometFN.  # noqa: E501
        :type: list[GrometBoxFunction]
        """

        self._b = b

    @property
    def opi(self):
        """Gets the opi of this GrometFN.  # noqa: E501

        opi: The Outer Port Inputs of the FN Outer Box (b)   # noqa: E501

        :return: The opi of this GrometFN.  # noqa: E501
        :rtype: list[GrometPort]
        """
        return self._opi

    @opi.setter
    def opi(self, opi):
        """Sets the opi of this GrometFN.

        opi: The Outer Port Inputs of the FN Outer Box (b)   # noqa: E501

        :param opi: The opi of this GrometFN.  # noqa: E501
        :type: list[GrometPort]
        """

        self._opi = opi

    @property
    def opo(self):
        """Gets the opo of this GrometFN.  # noqa: E501

        opo: The Outer Port Outputs of the FN Outer Box (b)   # noqa: E501

        :return: The opo of this GrometFN.  # noqa: E501
        :rtype: list[GrometPort]
        """
        return self._opo

    @opo.setter
    def opo(self, opo):
        """Sets the opo of this GrometFN.

        opo: The Outer Port Outputs of the FN Outer Box (b)   # noqa: E501

        :param opo: The opo of this GrometFN.  # noqa: E501
        :type: list[GrometPort]
        """

        self._opo = opo

    @property
    def wopio(self):
        """Gets the wopio of this GrometFN.  # noqa: E501

        wopio: The Wires from (src) Outer Box Outer Port Inputs (opi) to (tgt) Outer Box Outer Port Outputs (opo). (AKA: Passthrough.)   # noqa: E501

        :return: The wopio of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wopio

    @wopio.setter
    def wopio(self, wopio):
        """Sets the wopio of this GrometFN.

        wopio: The Wires from (src) Outer Box Outer Port Inputs (opi) to (tgt) Outer Box Outer Port Outputs (opo). (AKA: Passthrough.)   # noqa: E501

        :param wopio: The wopio of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wopio = wopio

    @property
    def bf(self):
        """Gets the bf of this GrometFN.  # noqa: E501

        bf: The GrometBoxFunctions within this GrometFN.   # noqa: E501

        :return: The bf of this GrometFN.  # noqa: E501
        :rtype: list[GrometBoxFunction]
        """
        return self._bf

    @bf.setter
    def bf(self, bf):
        """Sets the bf of this GrometFN.

        bf: The GrometBoxFunctions within this GrometFN.   # noqa: E501

        :param bf: The bf of this GrometFN.  # noqa: E501
        :type: list[GrometBoxFunction]
        """

        self._bf = bf

    @property
    def pif(self):
        """Gets the pif of this GrometFN.  # noqa: E501

        pif: The Port Inputs of the GrometBoxFunctions (bf).   # noqa: E501

        :return: The pif of this GrometFN.  # noqa: E501
        :rtype: list[GrometPort]
        """
        return self._pif

    @pif.setter
    def pif(self, pif):
        """Sets the pif of this GrometFN.

        pif: The Port Inputs of the GrometBoxFunctions (bf).   # noqa: E501

        :param pif: The pif of this GrometFN.  # noqa: E501
        :type: list[GrometPort]
        """

        self._pif = pif

    @property
    def pof(self):
        """Gets the pof of this GrometFN.  # noqa: E501

        pof: The Port Outputs of the GrometBoxFunctions (bf).   # noqa: E501

        :return: The pof of this GrometFN.  # noqa: E501
        :rtype: list[GrometPort]
        """
        return self._pof

    @pof.setter
    def pof(self, pof):
        """Sets the pof of this GrometFN.

        pof: The Port Outputs of the GrometBoxFunctions (bf).   # noqa: E501

        :param pof: The pof of this GrometFN.  # noqa: E501
        :type: list[GrometPort]
        """

        self._pof = pof

    @property
    def wfopi(self):
        """Gets the wfopi of this GrometFN.  # noqa: E501

        wfopi: The Wires from (src) GrometBoxFunctions Port Inputs (pif) to (tgt) Outer Box Outer Port Inputs (opi).   # noqa: E501

        :return: The wfopi of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wfopi

    @wfopi.setter
    def wfopi(self, wfopi):
        """Sets the wfopi of this GrometFN.

        wfopi: The Wires from (src) GrometBoxFunctions Port Inputs (pif) to (tgt) Outer Box Outer Port Inputs (opi).   # noqa: E501

        :param wfopi: The wfopi of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wfopi = wfopi

    @property
    def wfl(self):
        """Gets the wfl of this GrometFN.  # noqa: E501

        wfl: The Wires from (src) GrometBoxLoops Port Inputs (pil) to (tgt) GrometBoxFunctions Port Outputs (pof).   # noqa: E501

        :return: The wfl of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wfl

    @wfl.setter
    def wfl(self, wfl):
        """Sets the wfl of this GrometFN.

        wfl: The Wires from (src) GrometBoxLoops Port Inputs (pil) to (tgt) GrometBoxFunctions Port Outputs (pof).   # noqa: E501

        :param wfl: The wfl of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wfl = wfl

    @property
    def wff(self):
        """Gets the wff of this GrometFN.  # noqa: E501

        wff: The Wires from (src) GrometBoxFunctions Port Inputs (pif) to (tgt) GrometBoxFunctions Port Outputs (pof).   # noqa: E501

        :return: The wff of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wff

    @wff.setter
    def wff(self, wff):
        """Sets the wff of this GrometFN.

        wff: The Wires from (src) GrometBoxFunctions Port Inputs (pif) to (tgt) GrometBoxFunctions Port Outputs (pof).   # noqa: E501

        :param wff: The wff of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wff = wff

    @property
    def wfc(self):
        """Gets the wfc of this GrometFN.  # noqa: E501

        wfc: The Wires from (src) GrometBoxConditionals Port Inputs (pic) to (tgt) GrometBoxFunctions Port Outputs (pof).   # noqa: E501

        :return: The wfc of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wfc

    @wfc.setter
    def wfc(self, wfc):
        """Sets the wfc of this GrometFN.

        wfc: The Wires from (src) GrometBoxConditionals Port Inputs (pic) to (tgt) GrometBoxFunctions Port Outputs (pof).   # noqa: E501

        :param wfc: The wfc of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wfc = wfc

    @property
    def wfopo(self):
        """Gets the wfopo of this GrometFN.  # noqa: E501

        wfopo: The Wires from (src) Outer Box Outer Port Outputs (opo) to (tgt) GrometBoxFunctions Port Outputs (pof).   # noqa: E501

        :return: The wfopo of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wfopo

    @wfopo.setter
    def wfopo(self, wfopo):
        """Sets the wfopo of this GrometFN.

        wfopo: The Wires from (src) Outer Box Outer Port Outputs (opo) to (tgt) GrometBoxFunctions Port Outputs (pof).   # noqa: E501

        :param wfopo: The wfopo of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wfopo = wfopo

    @property
    def bl(self):
        """Gets the bl of this GrometFN.  # noqa: E501

        bl: The FN GrometBoxLoops within this GrometFN.   # noqa: E501

        :return: The bl of this GrometFN.  # noqa: E501
        :rtype: list[GrometBoxLoop]
        """
        return self._bl

    @bl.setter
    def bl(self, bl):
        """Sets the bl of this GrometFN.

        bl: The FN GrometBoxLoops within this GrometFN.   # noqa: E501

        :param bl: The bl of this GrometFN.  # noqa: E501
        :type: list[GrometBoxLoop]
        """

        self._bl = bl

    @property
    def pil(self):
        """Gets the pil of this GrometFN.  # noqa: E501

        pil: The Port Inputs of the GrometBoxLoops (bl)   # noqa: E501

        :return: The pil of this GrometFN.  # noqa: E501
        :rtype: list[GrometPort]
        """
        return self._pil

    @pil.setter
    def pil(self, pil):
        """Sets the pil of this GrometFN.

        pil: The Port Inputs of the GrometBoxLoops (bl)   # noqa: E501

        :param pil: The pil of this GrometFN.  # noqa: E501
        :type: list[GrometPort]
        """

        self._pil = pil

    @property
    def pol(self):
        """Gets the pol of this GrometFN.  # noqa: E501

        pol: The Port Outputs of the GrometBoxLoops (bl)   # noqa: E501

        :return: The pol of this GrometFN.  # noqa: E501
        :rtype: list[GrometPort]
        """
        return self._pol

    @pol.setter
    def pol(self, pol):
        """Sets the pol of this GrometFN.

        pol: The Port Outputs of the GrometBoxLoops (bl)   # noqa: E501

        :param pol: The pol of this GrometFN.  # noqa: E501
        :type: list[GrometPort]
        """

        self._pol = pol

    @property
    def wlopi(self):
        """Gets the wlopi of this GrometFN.  # noqa: E501

        wlopi: The Wires from (src) GrometBoxLoops Port Inputs (pil) to (tgt) the Outer Box Outer Port Inputs (opi).   # noqa: E501

        :return: The wlopi of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wlopi

    @wlopi.setter
    def wlopi(self, wlopi):
        """Sets the wlopi of this GrometFN.

        wlopi: The Wires from (src) GrometBoxLoops Port Inputs (pil) to (tgt) the Outer Box Outer Port Inputs (opi).   # noqa: E501

        :param wlopi: The wlopi of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wlopi = wlopi

    @property
    def wll(self):
        """Gets the wll of this GrometFN.  # noqa: E501

        wll: The Wires from (src) the GrometBoxLoops Port Inputs (pil) to (tgt) the GrometBoxLoops Port Outputs (pol).   # noqa: E501

        :return: The wll of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wll

    @wll.setter
    def wll(self, wll):
        """Sets the wll of this GrometFN.

        wll: The Wires from (src) the GrometBoxLoops Port Inputs (pil) to (tgt) the GrometBoxLoops Port Outputs (pol).   # noqa: E501

        :param wll: The wll of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wll = wll

    @property
    def wlf(self):
        """Gets the wlf of this GrometFN.  # noqa: E501

        wlf: The Wires from (src) the GrometBoxFunctions Port Inputs (pif) to (tgt) the GrometBoxLoops Port Outputs (pol).   # noqa: E501

        :return: The wlf of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wlf

    @wlf.setter
    def wlf(self, wlf):
        """Sets the wlf of this GrometFN.

        wlf: The Wires from (src) the GrometBoxFunctions Port Inputs (pif) to (tgt) the GrometBoxLoops Port Outputs (pol).   # noqa: E501

        :param wlf: The wlf of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wlf = wlf

    @property
    def wlc(self):
        """Gets the wlc of this GrometFN.  # noqa: E501

        wlc: The Wires from (src) the GrometBoxConditionals Port Inputs (pic) to (tgt) the GrometBoxLoops Port Outputs (pol).   # noqa: E501

        :return: The wlc of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wlc

    @wlc.setter
    def wlc(self, wlc):
        """Sets the wlc of this GrometFN.

        wlc: The Wires from (src) the GrometBoxConditionals Port Inputs (pic) to (tgt) the GrometBoxLoops Port Outputs (pol).   # noqa: E501

        :param wlc: The wlc of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wlc = wlc

    @property
    def wlopo(self):
        """Gets the wlopo of this GrometFN.  # noqa: E501

        wlopo: The Wires from (src) Outer Box Outer Port Outputs (opo) to (tgt) GrometBoxLoops Port Outputs (pol).   # noqa: E501

        :return: The wlopo of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wlopo

    @wlopo.setter
    def wlopo(self, wlopo):
        """Sets the wlopo of this GrometFN.

        wlopo: The Wires from (src) Outer Box Outer Port Outputs (opo) to (tgt) GrometBoxLoops Port Outputs (pol).   # noqa: E501

        :param wlopo: The wlopo of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wlopo = wlopo

    @property
    def bc(self):
        """Gets the bc of this GrometFN.  # noqa: E501

        bc: The FN GrometBoxConditionals within this GrometFN.   # noqa: E501

        :return: The bc of this GrometFN.  # noqa: E501
        :rtype: list[GrometBoxConditional]
        """
        return self._bc

    @bc.setter
    def bc(self, bc):
        """Sets the bc of this GrometFN.

        bc: The FN GrometBoxConditionals within this GrometFN.   # noqa: E501

        :param bc: The bc of this GrometFN.  # noqa: E501
        :type: list[GrometBoxConditional]
        """

        self._bc = bc

    @property
    def pic(self):
        """Gets the pic of this GrometFN.  # noqa: E501

        pic: The Port Inputs of the GrometBoxConditionals (bc)   # noqa: E501

        :return: The pic of this GrometFN.  # noqa: E501
        :rtype: list[GrometPort]
        """
        return self._pic

    @pic.setter
    def pic(self, pic):
        """Sets the pic of this GrometFN.

        pic: The Port Inputs of the GrometBoxConditionals (bc)   # noqa: E501

        :param pic: The pic of this GrometFN.  # noqa: E501
        :type: list[GrometPort]
        """

        self._pic = pic

    @property
    def poc(self):
        """Gets the poc of this GrometFN.  # noqa: E501

        poc: The Port Outputs of the GrometBoxConditionals (bc)   # noqa: E501

        :return: The poc of this GrometFN.  # noqa: E501
        :rtype: list[GrometPort]
        """
        return self._poc

    @poc.setter
    def poc(self, poc):
        """Sets the poc of this GrometFN.

        poc: The Port Outputs of the GrometBoxConditionals (bc)   # noqa: E501

        :param poc: The poc of this GrometFN.  # noqa: E501
        :type: list[GrometPort]
        """

        self._poc = poc

    @property
    def wcopi(self):
        """Gets the wcopi of this GrometFN.  # noqa: E501

        wcopi: The Wires from (src) the GrometBoxConditionals Port Inputs (pic) to (tgt) the Outer Box Outer Port Inputs (opi).   # noqa: E501

        :return: The wcopi of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wcopi

    @wcopi.setter
    def wcopi(self, wcopi):
        """Sets the wcopi of this GrometFN.

        wcopi: The Wires from (src) the GrometBoxConditionals Port Inputs (pic) to (tgt) the Outer Box Outer Port Inputs (opi).   # noqa: E501

        :param wcopi: The wcopi of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wcopi = wcopi

    @property
    def wcl(self):
        """Gets the wcl of this GrometFN.  # noqa: E501

        wcl: The Wires from (src) the GrometBoxLoops Port Inputs (pil) to (tgt) the GrometBoxConditionals Port Outputs (poc).   # noqa: E501

        :return: The wcl of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wcl

    @wcl.setter
    def wcl(self, wcl):
        """Sets the wcl of this GrometFN.

        wcl: The Wires from (src) the GrometBoxLoops Port Inputs (pil) to (tgt) the GrometBoxConditionals Port Outputs (poc).   # noqa: E501

        :param wcl: The wcl of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wcl = wcl

    @property
    def wcf(self):
        """Gets the wcf of this GrometFN.  # noqa: E501

        wcf: The Wires from (src) the GrometBoxFunctions Port Inputs (pif) to (tgt) the GrometBoxConditionals Port Outputs (poc).   # noqa: E501

        :return: The wcf of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wcf

    @wcf.setter
    def wcf(self, wcf):
        """Sets the wcf of this GrometFN.

        wcf: The Wires from (src) the GrometBoxFunctions Port Inputs (pif) to (tgt) the GrometBoxConditionals Port Outputs (poc).   # noqa: E501

        :param wcf: The wcf of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wcf = wcf

    @property
    def wcc(self):
        """Gets the wcc of this GrometFN.  # noqa: E501

        wcc: The Wires from (src) the GrometBoxConditionals Port Inputs (pic) to (tgt) the GrometBoxConditionals Port Outputs (poc).   # noqa: E501

        :return: The wcc of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wcc

    @wcc.setter
    def wcc(self, wcc):
        """Sets the wcc of this GrometFN.

        wcc: The Wires from (src) the GrometBoxConditionals Port Inputs (pic) to (tgt) the GrometBoxConditionals Port Outputs (poc).   # noqa: E501

        :param wcc: The wcc of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wcc = wcc

    @property
    def wcopo(self):
        """Gets the wcopo of this GrometFN.  # noqa: E501

        wcopo: The Wires from (src) the Outer Box Outer Port Outputs (opo) to (tgt) the GrometBoxConditionals Port Outputs (poc).   # noqa: E501

        :return: The wcopo of this GrometFN.  # noqa: E501
        :rtype: list[GrometWire]
        """
        return self._wcopo

    @wcopo.setter
    def wcopo(self, wcopo):
        """Sets the wcopo of this GrometFN.

        wcopo: The Wires from (src) the Outer Box Outer Port Outputs (opo) to (tgt) the GrometBoxConditionals Port Outputs (poc).   # noqa: E501

        :param wcopo: The wcopo of this GrometFN.  # noqa: E501
        :type: list[GrometWire]
        """

        self._wcopo = wcopo

    def to_dict(self):
        """Returns the model properties as a dict"""
        result = {}

        for attr, _ in six.iteritems(self.swagger_types):
            value = getattr(self, attr)
            if isinstance(value, list):
                result[attr] = list(map(
                    lambda x: x.to_dict() if hasattr(x, "to_dict") else x,
                    value
                ))
            elif hasattr(value, "to_dict"):
                result[attr] = value.to_dict()
            elif isinstance(value, dict):
                result[attr] = dict(map(
                    lambda item: (item[0], item[1].to_dict())
                    if hasattr(item[1], "to_dict") else item,
                    value.items()
                ))
            else:
                result[attr] = value
        if issubclass(GrometFN, dict):
            for key, value in self.items():
                result[key] = value

        return result

    def to_str(self):
        """Returns the string representation of the model"""
        return pprint.pformat(self.to_dict())

    def __repr__(self):
        """For `print` and `pprint`"""
        return self.to_str()

    def __eq__(self, other):
        """Returns true if both objects are equal"""
        if not isinstance(other, GrometFN):
            return False

        return self.__dict__ == other.__dict__

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        return not self == other
