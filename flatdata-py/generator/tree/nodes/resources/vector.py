from .base import ResourceBase
from generator.tree.nodes.references import StructureReference


class Vector(ResourceBase):
    def __init__(self, name, properties=None, type=None):
        super(Vector, self).__init__(name=name, properties=properties)
        self._type = type

    @staticmethod
    def create(properties):
        return Vector(name=properties.name,
                      properties=properties,
                      type=properties.type.vector.type)

    def _create_references(self):
        return [StructureReference(name=self._type)]