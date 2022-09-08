import serial
import serial.tools.list_ports

""" Serial Communicator class """
class Communicator:
    def __init__(self):
        self.serial = serial.Serial()

    """
    Sets the serial port for the 
    Serial object
    """
    def set_port(self, port):
        self.serial.port = port

    """
    Opens a connection to the serial port
    This needs to be called before any communication
    """
    def open(self):
        self.serial.open()

    """
    Returns a list of available serial ports
    """
    def list_ports(self):
        return serial.tools.list_ports.comports()

    """
    Reads a byte from the serial port
    """
    def read(self):
        return self.serial.read()

    """
    Reads from the serial port until an
    end line character is reached
    """
    def readline(self):
        return self.serial.readline()

    def write(self):
        pass
