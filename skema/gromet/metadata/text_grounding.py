# coding: utf-8

"""
    GroMEt Metadata spec

    Grounded Model Exchange (GroMEt) Metadata schema specification  __Using Swagger to Generate Class Structure__  To automatically generate Python or Java models corresponding to this document, you can use [swagger-codegen](https://swagger.io/tools/swagger-codegen/). We can use this to generate client code based off of this spec that will also generate the class structure.  1. Install via the method described for your operating system    [here](https://github.com/swagger-api/swagger-codegen#Prerequisites).    Make sure to install a version after 3.0 that will support openapi 3. 2. Run swagger-codegen with the options in the example below.    The URL references where the yaml for this documentation is stored on    github. Make sure to replace CURRENT_VERSION with the correct version.    (The current version is `0.1.4`.)    To generate Java classes rather, change the `-l python` to `-l java`.    Change the value to the `-o` option to the desired output location.    ```    swagger-codegen generate -l python -o ./client -i https://raw.githubusercontent.com/ml4ai/automates-v2/master/docs/source/gromet_metadata_v{CURRENT_VERSION}.yaml    ``` 3. Once it executes, the client code will be generated at your specified    location.    For python, the classes will be located in    `$OUTPUT_PATH/swagger_client/models/`.    For java, they will be located in    `$OUTPUT_PATH/src/main/java/io/swagger/client/model/`  If generating GroMEt Metadata schema data model classes in SKEMA (AutoMATES), then after generating the above, follow the instructions here: ``` <automates>/automates/model_assembly/gromet/metadata/README.md ```   # noqa: E501

    OpenAPI spec version: 0.1.6
    Contact: claytonm@arizona.edu
    Generated by: https://github.com/swagger-api/swagger-codegen.git
"""

import pprint
import re  # noqa: F401

import six

class TextGrounding(object):
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
        'argument_name': 'str',
        'id': 'str',
        'description': 'str',
        'score': 'float'
    }

    attribute_map = {
        'argument_name': 'argument_name',
        'id': 'id',
        'description': 'description',
        'score': 'score'
    }

    def __init__(self, argument_name=None, id=None, description=None, score=None):  # noqa: E501
        """TextGrounding - a model defined in Swagger"""  # noqa: E501
        self._argument_name = None
        self._id = None
        self._description = None
        self._score = None
        self.discriminator = None
        if argument_name is not None:
            self.argument_name = argument_name
        if id is not None:
            self.id = id
        if description is not None:
            self.description = description
        if score is not None:
            self.score = score

    @property
    def argument_name(self):
        """Gets the argument_name of this TextGrounding.  # noqa: E501


        :return: The argument_name of this TextGrounding.  # noqa: E501
        :rtype: str
        """
        return self._argument_name

    @argument_name.setter
    def argument_name(self, argument_name):
        """Sets the argument_name of this TextGrounding.


        :param argument_name: The argument_name of this TextGrounding.  # noqa: E501
        :type: str
        """

        self._argument_name = argument_name

    @property
    def id(self):
        """Gets the id of this TextGrounding.  # noqa: E501


        :return: The id of this TextGrounding.  # noqa: E501
        :rtype: str
        """
        return self._id

    @id.setter
    def id(self, id):
        """Sets the id of this TextGrounding.


        :param id: The id of this TextGrounding.  # noqa: E501
        :type: str
        """

        self._id = id

    @property
    def description(self):
        """Gets the description of this TextGrounding.  # noqa: E501


        :return: The description of this TextGrounding.  # noqa: E501
        :rtype: str
        """
        return self._description

    @description.setter
    def description(self, description):
        """Sets the description of this TextGrounding.


        :param description: The description of this TextGrounding.  # noqa: E501
        :type: str
        """

        self._description = description

    @property
    def score(self):
        """Gets the score of this TextGrounding.  # noqa: E501


        :return: The score of this TextGrounding.  # noqa: E501
        :rtype: float
        """
        return self._score

    @score.setter
    def score(self, score):
        """Sets the score of this TextGrounding.


        :param score: The score of this TextGrounding.  # noqa: E501
        :type: float
        """

        self._score = score

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
        if issubclass(TextGrounding, dict):
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
        if not isinstance(other, TextGrounding):
            return False

        return self.__dict__ == other.__dict__

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        return not self == other
