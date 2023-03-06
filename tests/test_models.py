import numpy as np
import pandas
from pywr import run_model_from_path
from pathlib import Path
import h5py
import pytest


@pytest.fixture()
def test_dir() -> Path:
    return Path(__file__).parent


@pytest.fixture()
def model_dir(test_dir: Path):
    return test_dir / "models"


def test_simple_timeseries(model_dir: Path, tmpdir: Path):
    """Test the simple model"""

    filename = model_dir / "simple-timeseries" / "model.json"

    output_fn = tmpdir / "output.h5"

    run_model_from_path(filename, "clp", output_h5=str(output_fn))
    assert output_fn.exists()

    expected_data = pandas.read_csv(
        model_dir / "simple-timeseries" / "expected.csv", index_col=0
    )

    with h5py.File(output_fn, "r") as fh:
        for node, df in expected_data.items():
            np.testing.assert_allclose(np.squeeze(fh[node]), df)


# TODO these tests could be auto-discovered.
@pytest.mark.parametrize(
    "model_name",
    [
        "simple-timeseries",
        "simple-storage-timeseries",
        # "simple-wasm", TODO the schema for the WASM parameter needs implementing
        "aggregated-node1",
        "piecewise-link1",
    ],
)
def test_model(model_dir: Path, tmpdir: Path, model_name: str):

    filename = model_dir / model_name / "model.json"
    output_fn = tmpdir / "output.h5"

    run_model_from_path(filename, "clp", output_h5=str(output_fn))
    assert output_fn.exists()

    expected_fn = model_dir / model_name / "expected.csv"
    if not expected_fn.exists():
        expected_fn = model_dir / model_name / "expected.csv.gz"

    expected_data = pandas.read_csv(expected_fn, index_col=0)

    with h5py.File(output_fn, "r") as fh:
        print(fh.keys())
        for node, df in expected_data.items():
            print(node, df)
            np.testing.assert_allclose(np.squeeze(fh[node]), df)
