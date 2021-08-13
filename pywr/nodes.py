from __future__ import annotations
from pathlib import Path
from typing import List, Optional, Dict, Union, Iterable
from pydantic import BaseModel
from .pywr import PyModel, ParameterNotFoundError  # type: ignore
from .parameters import ParameterCollection, ParameterRef
from .recorders import RecorderCollection
from .tables import TableCollection
import json
import yaml


_node_registry = {}
_output_registry = {}


class BaseNode(BaseModel):
    name: str
    comment: Optional[str] = None

    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        _node_registry[cls.__name__.lower()] = cls

    def create_nodes(self, r_model: PyModel):
        raise NotImplementedError()

    def set_constraints(self, r_model: PyModel):
        raise NotImplementedError()

    @classmethod
    def get_class(cls, node_type: str) -> BaseNode:
        return _node_registry[node_type.lower() + "node"]

    @classmethod
    def from_data(cls, node_data) -> BaseNode:
        klass = cls.get_class(node_data.pop("type"))
        return klass(**node_data)

    @classmethod
    def __get_validators__(cls):
        yield cls.validate

    @classmethod
    def validate(cls, data):
        if not isinstance(data, dict):
            raise TypeError("dict required")
        if "type" not in data:
            raise ValueError('"type" key required')

        klass = cls.get_class(data.pop("type"))
        return klass(**data)


class InputNode(BaseNode):
    cost: Optional[ParameterRef] = None
    min_flow: Optional[ParameterRef] = None
    max_flow: Optional[ParameterRef] = None

    def create_nodes(self, r_model: PyModel):
        r_model.add_input_node(self.name)

    def set_constraints(self, r_model: PyModel):
        if self.cost is not None:
            r_model.set_node_cost(self.name, self.cost)
        if self.max_flow is not None:
            r_model.set_node_constraint(self.name, "max_flow", self.max_flow)


class LinkNode(BaseNode):
    cost: Optional[ParameterRef] = None
    min_flow: Optional[ParameterRef] = None
    max_flow: Optional[ParameterRef] = None

    def create_nodes(self, r_model: PyModel):
        r_model.add_link_node(self.name)

    def set_constraints(self, r_model: PyModel):
        if self.cost is not None:
            r_model.set_node_cost(self.name, self.cost)
        if self.max_flow is not None:
            r_model.set_node_constraint(self.name, "max_flow", self.max_flow)


class OutputNode(BaseNode):
    cost: Optional[ParameterRef] = None
    min_flow: Optional[ParameterRef] = None
    max_flow: Optional[ParameterRef] = None

    def create_nodes(self, r_model: PyModel):
        r_model.add_output_node(self.name)

    def set_constraints(self, r_model: PyModel):
        if self.cost is not None:
            r_model.set_node_cost(self.name, self.cost)
        if self.max_flow is not None:
            r_model.set_node_constraint(self.name, "max_flow", self.max_flow)


class StorageNode(BaseNode):
    cost: Optional[ParameterRef] = None
    initial_volume: float = 0.0
    min_volume: Optional[ParameterRef] = None
    max_volume: Optional[ParameterRef] = None

    def create_nodes(self, r_model: PyModel):
        r_model.add_storage_node(self.name, self.initial_volume)

    def set_constraints(self, r_model: PyModel):
        if self.cost is not None:
            r_model.set_node_cost(self.name, self.cost)
        if self.max_volume is not None:
            r_model.set_node_constraint(self.name, "max_volume", self.max_volume)


class Edge(BaseModel):
    from_node: str
    to_node: str

    def create_edge(self, r_model: PyModel):
        r_model.connect_nodes(self.from_node, self.to_node)


class NodeCollection:
    def __init__(self):
        self._nodes: Dict[str, BaseNode] = {}

    def __getitem__(self, item: str):
        return self._nodes[item]

    def __setitem__(self, key: str, value: BaseNode):
        self._nodes[key] = value

    def __iter__(self):
        return iter(self._nodes.values())

    def __len__(self):
        return len(self._nodes)

    def __contains__(self, item):
        return item in self._nodes

    @classmethod
    def __get_validators__(cls):
        yield cls.validate

    @classmethod
    def validate(cls, data):
        if not isinstance(data, list):
            raise TypeError("list required")

        collection = cls()
        for node_data in data:

            if "type" not in node_data:
                raise ValueError('"type" key required')

            node = BaseNode.from_data(node_data)
            if node.name in collection:
                raise ValueError(f'Node name "{node.name}" already defined.')
            collection[node.name] = node
        return collection

    def append(self, node: BaseNode):
        self._nodes[node.name] = node

    def extend(self, nodes: Iterable[BaseNode]):
        for node in nodes:
            self.append(node)


class PrintRecorder:
    def save(self, *args):
        print("saving", args)


class BaseOutput(BaseModel):
    name: str

    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        _output_registry[cls.__name__.lower()] = cls

    def create_output(self, r_model: PyModel):
        raise NotImplementedError


class HDF5Output(BaseOutput):
    filename: Path

    def create_output(self, r_model: PyModel):
        r_model.add_hdf5_output(self.name, str(self.filename))


class OutputCollection:
    def __init__(self):
        self._outputs: Dict[str, BaseOutput] = {}

    def __getitem__(self, item: str):
        return self._outputs[item]

    def __setitem__(self, key: str, value: BaseOutput):
        self._outputs[key] = value

    def __iter__(self):
        return iter(self._outputs.values())

    def __len__(self):
        return len(self._outputs)

    def __contains__(self, item):
        return item in self._outputs

    def insert(self, value: BaseOutput):
        self[value.name] = value

    @classmethod
    def __get_validators__(cls):
        yield cls.validate

    @classmethod
    def validate(cls, data):
        if not isinstance(data, list):
            raise TypeError("list required")

        collection = cls()
        for output_data in data:

            if "type" not in output_data:
                raise ValueError('"type" key required')

            klass_name = output_data.pop("type") + "output"
            klass = _node_registry[klass_name]
            output = klass(**output_data)
            if output.name in collection:
                raise ValueError(f'Output name "{output.name}" already defined.')
            collection[output.name] = output
        return collection


class Timestepper(BaseModel):
    start: str
    end: str
    timestep: int


class Model(BaseModel):
    timestepper: Timestepper
    nodes: NodeCollection = NodeCollection()
    edges: List[Edge] = []
    parameters: ParameterCollection = ParameterCollection()
    recorders: RecorderCollection = RecorderCollection()
    tables: TableCollection = TableCollection()
    outputs: OutputCollection = OutputCollection()
    path: Optional[Path] = None  # TODO not sure about this one.

    @classmethod
    def from_file(cls, filepath: Path) -> Model:
        """Load a model from a file."""

        ext = filepath.suffix.lower()
        if ext == ".json":
            model = cls.from_json(filepath)
        elif ext in (".yaml", ".yml"):
            model = cls.from_yaml(filepath)
        else:
            raise ValueError(f'Filetype "{ext}" not supported.')
        return model

    @classmethod
    def from_json(cls, filepath: Path) -> Model:
        """Load a model from a JSON file."""
        with open(filepath) as fh:
            data = json.load(fh)
        return cls(path=filepath.parent, **data)

    @classmethod
    def from_yaml(cls, filepath: Path) -> Model:
        """Load a model from a YAML file."""
        with open(filepath) as fh:
            data = yaml.safe_load(fh)
        return cls(path=filepath.parent, **data)

    def build(self) -> PyModel:
        """Construct a `PyModel`"""

        r_model = PyModel()
        for node in self.nodes:
            node.create_nodes(r_model)

        for edge in self.edges:
            edge.create_edge(r_model)

        # Build the parameters ...
        remaining_parameters = [p for p in self.parameters]
        while len(remaining_parameters) > 0:
            failed_parameters = []  # Collection for parameters that fail to load
            for parameter in remaining_parameters:
                try:
                    parameter.create_parameter(r_model, self.path)
                except ParameterNotFoundError:
                    # Parameter failed due to not finding another parameter.
                    # This is likely a dependency that is not yet loaded.
                    failed_parameters.append(parameter)

            if len(failed_parameters) >= len(remaining_parameters):
                raise RuntimeError(
                    "Failed to load parameters due to a cycle in the dependency tree."
                )
            remaining_parameters = failed_parameters

        for recorder in self.recorders:
            recorder.create_recorder(r_model)

        for output in self.outputs:
            output.create_output(r_model)

        for node in self.nodes:
            node.set_constraints(r_model)

        # r_model.add_python_recorder("a-recorder", "NodeInFlow", 0, PrintRecorder())

        return r_model

    def run(self):
        r_model = self.build()
        r_model.run(
            "clp",
            self.timestepper.start,
            self.timestepper.end,
            self.timestepper.timestep,
        )
