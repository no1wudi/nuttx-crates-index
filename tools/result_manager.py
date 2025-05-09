# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

"""
JSON result manager for NuttX build utility.

This module provides functionality to manage build results in JSON format,
including reading existing results and writing new results to a JSON file.
"""

import os
import json


class JsonResultManager:
    """
    Class to manage reading and writing build results to JSON files.

    This class handles the persistence of build results, including baseline and
    crate-specific builds, allowing for tracking size differences across builds.
    It provides methods to append new build results and write the accumulated
    data to a JSON file for later analysis. If no JSON file is specified,
    the manager will collect data in memory but will not persist it.
    """

    def __init__(self, json_file):
        """
        Initialize the JsonResultManager with a file path and parse the JSON content.

        Args:
            json_file: Path to the JSON file to read from and write to, or None to collect data without saving
        """
        self.json_file = json_file
        self.data = self._parse_json()

    def _parse_json(self):
        """
        Parse the JSON file into a Python dictionary structure.

        Returns:
            dict: A dictionary containing all build results, with at least a "builds" list.
                 Returns {"builds": []} if the file doesn't exist or contains invalid JSON.
        """
        return {"builds": []}

    def append_result(
        self,
        crate_name,
        baseline,
        crate_size,
        diffs,
        timestamp,
        test_time=None,
        test_output=None,
        test_success=None,
        memory_leaked=None,
    ):
        """
        Append build results to the JSON data structure.

        This method adds a new build result entry to the data structure with information
        about the crate name, baseline and crate build sizes,
        the differences between them, a timestamp for when the build was performed,
        and test results if available.

        Args:
            crate_name (str): Name of the crate being built
            baseline (dict): Baseline build sizes containing text, data, bss, and total
            crate_size (dict): Current crate build sizes containing text, data, bss, and total
            diffs (dict): Size differences between baseline and crate build
            timestamp (int): Unix timestamp for the build session
            test_time (float, optional): Execution time of the test in seconds
            test_output (str, optional): Output of the test execution
            test_success (bool or str, optional): Whether the test was successful ("true", "false", or "skip")
            memory_leaked (int, optional): Amount of memory leaked during the test execution
        """
        # Add new build result
        build_data = {
            "crate_name": crate_name,
            "baseline": baseline,
            "crate_build": crate_size,
            "differences": {
                "text": diffs["text"],
                "data": diffs["data"],
                "bss": diffs["bss"],
                "total": diffs["total"],
            },
            "timestamp": timestamp,
        }

        # Convert test_success to string format
        if test_success is None:
            success_str = "skip"
        elif isinstance(test_success, bool):
            success_str = "pass" if test_success else "fail"
        else:
            success_str = str(test_success)  # Preserve string values

        # Always include test results with default values if not provided
        build_data["test"] = {
            "execution_time": test_time if test_time is not None else 0,
            "output": test_output if test_output is not None else "",
            "success": success_str,
            "memory_leaked": memory_leaked if memory_leaked is not None else 0,
        }

        self.data["builds"].append(build_data)

    def flush(self):
        """
        Write the current data back to the JSON file.

        This method serializes the internal data structure to JSON format
        and writes it to the file specified during initialization.
        If no file was specified, the data is discarded.
        """
        if not self.json_file:
            # No file specified, discard data
            return

        with open(self.json_file, "w") as f:
            json.dump(self.data, f, indent=2)

    def get_results(self):
        """
        Get the parsed JSON data structure.

        Returns:
            dict: A dictionary containing all build results, with at least a "builds" list
                 holding all the build data entries.
        """
        return self.data
