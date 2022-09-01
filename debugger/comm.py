import serial
import serial.tools.list_ports

""" Serial Communicator class """
class Communicator:
    def __init__(self):
        self.serial = serial.Serial()

    def set_port(self, port):
        self.serial.port = port

    def open(self):
        self.serial.open()

    def list_ports(self):
        return serial.tools.list_ports.comports()

    def read(self):
        return self.serial.read()

    def readline(self):
        return self.serial.readline()

    def write(self):
        pass
