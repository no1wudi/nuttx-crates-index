# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

"""
NuttX binary runner module.

This module provides functionality to execute NuttX binaries using
different emulation tools like QEMU and Renode.
"""

import pexpect
import time

# Board configurations with QEMU parameters
QEMU_BOARD_CONFIGS = {
    "mps2-an521:nsh": {
        "cmd": "qemu-system-arm -M mps2-an521 -nographic -kernel {kernel}",
    },
    "sabre-6quad:nsh": {
        "cmd": "qemu-system-arm -M sabrelite -nographic -kernel {kernel}",
    },
    "rv-virt:nsh": {
        "cmd": "qemu-system-riscv32 -M virt,aclint=on -cpu rv32 -smp 8 -bios none -nographic -kernel {kernel}",
    },
    "rv-virt:nsh64": {
        "cmd": "qemu-system-riscv64 -M virt,aclint=on -cpu rv64 -smp 8 -bios none -nographic -kernel {kernel}",
    },
}


class Runner:
    """Class to run NuttX binaries using QEMU."""

    def __init__(self, binary_path: str, board: str = "mps2-an521"):
        """
        Initialize Runner with binary path and board name.

        Args:
            binary_path: Path to the binary file to run
            board: Name of the board to emulate (must be in QEMU_BOARD_CONFIGS)
        """
        if board not in QEMU_BOARD_CONFIGS:
            raise ValueError(
                f"Unsupported board: {board}. Supported boards: {list(QEMU_BOARD_CONFIGS.keys())}"
            )

        self.binary_path = binary_path
        self.board = board
        self.process = None  # Explicitly initialize process as None
        self.timeout = 5  # Default timeout in seconds
        self.prompt = "nsh>"

    def _build_qemu_command(self) -> str:
        """Build the QEMU command based on board configuration."""
        config = QEMU_BOARD_CONFIGS[self.board]

        # Replace {kernel} with the actual binary path
        cmd = config["cmd"].format(kernel=self.binary_path)

        return cmd

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

    def send_command(self, command: str):
        """
        Send a command to the QEMU process and wait for echo confirmation.

        Args:
            command (str): The command string to send to QEMU

        Returns:
            bool or None:
                - True if command was sent and echoed successfully
                - False if prompt appeared before command echo (unexpected behavior)
                - None if command was not found

        Raises:
            RuntimeError: If QEMU process is not running
        """
        """Send a command to the QEMU process."""
        if not self.process or not self.process.isalive():
            raise RuntimeError("QEMU process is not running")

        self.process.sendline(command)
        # Wait for the command echo to confirm it was sent
        index = self.process.expect(
            [command, f"{command}: command not found", self.prompt], timeout=1
        )
        if index == 0:
            # Command was sent successfully
            return True
        elif index == 1:
            # Command not found
            return None
        elif index == 2:
            # Prompt appeared before command echo, unexpected behavior
            return False

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

    def get_free_memory(self) -> int:
        """
        Get the free memory from the NuttX shell.

        This method sends the "free" command to the NuttX shell and parses
        the output to extract the free memory value.

        Returns:
            int: Free memory in bytes

        Raises:
            RuntimeError: If there's an issue with the QEMU process
            ValueError: If output cannot be parsed
        """
        if not self.process or not self.process.isalive():
            raise RuntimeError("QEMU process is not running")

        self.send_command("free")
        output = self.read_output(timeout=5)

        # Parse the output lines
        for line in output.splitlines():
            if "Umem" in line:  # Look for the line containing memory info
                fields = line.split()
                if len(fields) >= 4:  # Ensure we have enough fields
                    return int(fields[2])  # Return the "free" column value

        raise ValueError("Could not parse free memory from output")

    def run(self, command: str, timeout: float = None):
        """
        Run a command on the NuttX shell and collect the results.

        This method sends a command to the NuttX shell, waits for execution to complete,
        captures the output, and determines whether the command executed successfully.
        If the QEMU process is not already running, it will be started automatically.
        The method also monitors memory usage before and after command execution.

        Args:
            command: The shell command to execute in the NuttX environment
            timeout: Maximum time in seconds to wait for the command to complete.
                     If None, uses the default timeout set for this Runner instance.

        Returns:
            dict: Contains:
                - execution_time (float): Time taken to execute the command in seconds
                - output (str): The command output text from the NuttX shell
                - success (bool): True if the command executed without errors
                - free_memory_before (int): Available memory before command execution in bytes
                - free_memory_after (int): Available memory after command execution in bytes

        Raises:
            RuntimeError: If there's an issue with the QEMU process
        """
        if timeout is None:
            timeout = self.timeout

        free_memory = 0
        free_memory_after = 0

        try:
            # Run the command and measure its execution time
            start_time = time.time()

            # Start the QEMU process
            self.start()

            # Get free memory before running the command
            free_memory = self.get_free_memory()

            self.send_command(command)
            output = self.read_output(timeout=timeout)

            success = True

            # Get free memory after running the command
            free_memory_after = self.get_free_memory()

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

        return {
            "execution_time": execution_time,
            "output": output,
            "success": success,
            "free_memory_before": free_memory,
            "free_memory_after": free_memory_after,
        }


if __name__ == "__main__":
    # Example usage
    import argparse

    parser = argparse.ArgumentParser(description="Run NuttX binaries with QEMU")
    parser.add_argument("binary", help="Path to NuttX binary file")
    parser.add_argument("--board", help="Board to emulate")
    parser.add_argument("--command", default="help", help="Command to run")

    args = parser.parse_args()

    runner = Runner(args.binary, args.board)
    result = runner.run(args.command)

    print(f"Command execution time: {result['execution_time']:.2f} seconds")
    print(f"Success: {result['success']}")
    print("Output:")
    print(result["output"])

    runner.stop()
