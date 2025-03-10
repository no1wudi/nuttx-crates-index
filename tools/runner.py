# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

"""
NuttX binary runner module.

This module provides functionality to execute NuttX binaries using
different emulation tools like QEMU and Renode.
"""

import pexpect
import time


class Runner:
    """Class to run NuttX binaries using QEMU."""

    # Board configurations with QEMU parameters
    QEMU_BOARD_CONFIGS = {
        "mps2-an521": {
            "machine": "mps2-an521",
            "nographic": True,
        },
        "sabre-6quad": {
            "machine": "sabrelite",
            "nographic": True,
        },
    }

    def __init__(self, binary_path: str, board: str = "mps2-an521"):
        """
        Initialize Runner with binary path and board name.

        Args:
            binary_path: Path to the binary file to run
            board: Name of the board to emulate (must be in QEMU_BOARD_CONFIGS)
        """
        if board not in self.QEMU_BOARD_CONFIGS:
            raise ValueError(
                f"Unsupported board: {board}. Supported boards: {list(self.QEMU_BOARD_CONFIGS.keys())}"
            )

        self.binary_path = binary_path
        self.board = board
        self.process = None  # Explicitly initialize process as None
        self.timeout = 5  # Default timeout in seconds
        self.prompt = "nsh>"

    def _build_qemu_command(self) -> str:
        """Build the QEMU command based on board configuration."""
        config = self.QEMU_BOARD_CONFIGS[self.board]
        cmd = ["qemu-system-arm"]

        # Add board specific configurations
        cmd.extend(["-machine", config["machine"]])

        if "cpu" in config:
            cmd.extend(["-cpu", config["cpu"]])

        if config.get("nographic", False):
            cmd.append("-nographic")

        if "serial" in config:
            cmd.extend(["-serial", config["serial"]])

        if "mem" in config:
            cmd.extend(["-m", config["mem"]])

        # Add the binary file
        cmd.extend(["-kernel", self.binary_path])

        return " ".join(cmd)

    def start(self) -> pexpect.spawn:
        """Start QEMU process and return the pexpect object."""
        cmd = self._build_qemu_command()

        # Spawn QEMU process with pexpect
        self.process = pexpect.spawn(cmd, encoding="utf-8", timeout=self.timeout)

        # Wait for NuttShell prompt to appear
        try:
            self.process.expect("NuttShell ", timeout=10)
            self.process.expect(self.prompt, timeout=5)
        except pexpect.TIMEOUT:
            raise RuntimeError("Failed to boot NuttX or find shell prompt")

        return self.process

    def stop(self):
        """Stop the QEMU process."""
        if self.process and self.process.isalive():
            try:
                # Send Ctrl+A x to quit QEMU
                self.process.sendcontrol("a")
                self.process.send("x")
                self.process.expect(pexpect.EOF, timeout=5)
            except pexpect.TIMEOUT:
                # Force kill if not responding
                self.process.terminate(force=True)
            finally:
                self.process.close()
                self.process = None

    def send_command(self, command: str) -> None:
        """Send a command to the QEMU process."""
        if not self.process or not self.process.isalive():
            raise RuntimeError("QEMU process is not running")

        self.process.sendline(command)
        # Wait for the command echo to confirm it was sent
        self.process.expect(command, timeout=1)

    def read_output(self, timeout: float = None) -> str:
        """
        Read output from QEMU process until the shell prompt is found.

        Args:
            timeout: Maximum time to wait for output

        Returns:
            Captured output as string
        """
        if not self.process or not self.process.isalive():
            raise RuntimeError("QEMU process is not running")

        if timeout is None:
            timeout = self.timeout

        try:
            # Wait for the prompt to appear
            index = self.process.expect(
                [self.prompt, "stack_dump", pexpect.TIMEOUT, pexpect.EOF],
                timeout=timeout,
            )

            if index == 0:  # Prompt found
                # Return everything up to the prompt
                return self.process.before.strip()
            elif index == 1:  # Stack dump
                return f"{self.process.before.strip()}\n[Crash detected]"
            elif index == 2:  # Timeout
                return f"{self.process.before.strip()}\n[Command timed out after {timeout}s]"
            else:  # EOF
                return (
                    f"{self.process.before.strip()}\n[Process terminated unexpectedly]"
                )

        except Exception as e:
            return f"Error reading output: {str(e)}"

    def run(self, command: str, timeout: float = None):
        """
        Run a command on the NuttX shell and collect the results.

        This method sends a command to the NuttX shell, waits for execution to complete,
        captures the output, and determines whether the command executed successfully.
        If the QEMU process is not already running, it will be started automatically.

        Args:
            command: The shell command to execute in the NuttX environment
            timeout: Maximum time in seconds to wait for the command to complete.
                     If None, uses the default timeout set for this Runner instance.

        Returns:
            tuple: Contains:
                - execution_time (float): Time taken to execute the command in seconds
                - output_text (str): The command output text from the NuttX shell
                - success_status (bool): True if the command executed without errors

        Raises:
            RuntimeError: If there's an issue with the QEMU process
        """
        if timeout is None:
            timeout = self.timeout

        try:
            # Run the command and measure its execution time
            start_time = time.time()

            # Start the QEMU process
            self.start()

            self.send_command(command)
            output = self.read_output(timeout=timeout)

            success = True

            # Check for success
            if "[Command timed out after" in output:
                success = False
            elif "[Crash detected]" in output:
                success = False

        except pexpect.TIMEOUT:
            output = "Command timed out"
            success = False
        except pexpect.EOF:
            output = "Process terminated unexpectedly"
            success = False
        except Exception as e:
            output = f"Error running command: {str(e)}"
            success = False

        execution_time = time.time() - start_time
        self.stop()
        return execution_time, output, success


if __name__ == "__main__":
    # Example usage
    import argparse

    parser = argparse.ArgumentParser(description="Run NuttX binaries with QEMU")
    parser.add_argument("binary", help="Path to NuttX binary file")
    parser.add_argument("--board", help="Board to emulate")
    parser.add_argument("--command", default="help", help="Command to run")

    args = parser.parse_args()

    runner = Runner(args.binary, args.board)
    execution_time, output, success = runner.run(args.command)

    print(f"Command execution time: {execution_time:.2f} seconds")
    print(f"Success: {success}")
    print("Output:")
    print(output)

    runner.stop()
