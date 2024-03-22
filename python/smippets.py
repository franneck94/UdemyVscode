"""
This module description would be similar to the class description
"""


class MyClass:
    """
    Class Description
    """

    val1: int
    val2: int

    def __init__(self, val1: int = None, val2: int = None) -> None:
        """
        Class Initialization Function. Gets called when the object is created

        Parameters
        ----------
        val1 (int) : Parameter Description
        val2 (int) : Parameter Description

        Raises
        ------
        ValueError
            If the value of val1 is None

        """
        if self.val1 is None and self.val2 is None:
            raise ValueError("val1 and val2 cannot be None")

        self.val1 = val1
        self.val2 = val2

    def __dict__(self) -> dict:
        """
        Dictionary format of the class

        Returns
        -------
        dict : The object in its JSON format

        """
        return {"val1": self.val1, "val2": self.val2}

    def __eq__(self, another_object: MyClass) -> bool:
        """
        Checking equality between two objects of MyClass

        Parameters
        ----------
        another_object (MyClass) : Parameter Description

        Returns
        -------
        bool : Whether the compared objects are same

        """
        are_objects_equal = True
        my_dict = dict(self)
        another_object_dict = dict(another_object)

        for key in my_dict:
            if my_dict[key] != another_object_dict[key]:
                are_objects_equal = False
                break

        return are_objects_equal


from enum import Enum


class MyEnum(Enum):
    """
    Enum Description here
    """

    RED = "red"
    GREEN = "green"
    BLUE = "blue"
