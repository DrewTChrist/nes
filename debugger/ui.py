import tkinter as tk
from tkinter import ttk
import queue

class UI(tk.Frame):
    def __init__(self, master, queue, ports, endCommand):
        super().__init__(master)
        self.queue = queue
        self.exit = endCommand
        self.started = False
        self.ports = ports
        self.init_ui(ports)

    def handle_message(self):
        while self.queue.qsize():
            try:
                msg = self.queue.get(0)
                self.update_reg_tree("a", msg)
            except queue.Empty:
                pass

    def init_ui(self, ports):
        self.pack(fill=tk.BOTH, expand=True)
        self.menu = tk.Menu(self)
        self.menu.add_command(label="Dump", command=None)
        #self.menu.add_command(label="Exit", command=self.master.quit)
        self.menu.add_command(label="Exit", command=self.exit)
        self.master.config(menu=self.menu)

        # Top frame
        self.top_frame = tk.Frame(self, background="#dddddd")
        self.top_frame.pack(fill=tk.BOTH, side=tk.TOP, expand=True)

        # Bottom frame
        self.bot_frame = tk.Frame(self, background="#cccccc")
        self.bot_frame.pack(fill=tk.BOTH, side=tk.BOTTOM, expand=True)

        # Registers frame top left
        self.left_frame = tk.Frame(self.top_frame, background="#eeeeee")
        self.left_frame.pack(fill=tk.BOTH, side=tk.LEFT, expand=True)

        # Top Center frame
        self.frame4 = tk.Frame(self.top_frame, background="#bbbbbb")
        self.frame4.pack(fill=tk.BOTH, side=tk.LEFT, expand=True)
        self.f4label = tk.Label(self.frame4, text="Stack Trace")
        self.f4label.pack(fill=tk.X)

        # Top Right frame
        self.frame5 = tk.Frame(self.top_frame, background="#aaaaaa")
        self.frame5.pack(fill=tk.BOTH, side=tk.LEFT, expand=True)
        self.instructions_label = tk.Label(self.frame5, text="Instructions")
        self.instructions_label.pack(fill=tk.X)

        self.frame6 = tk.Frame(self.bot_frame, background="#999999")
        self.frame6.pack(fill=tk.BOTH, side=tk.LEFT, expand=True)
        self.ports_label = tk.Label(self.frame6, text="Serial Ports")
        self.ports_label.pack(fill=tk.X)

        self.port_list = tk.Listbox(self.frame6)
        self.port_list.pack(fill=tk.BOTH, expand=True)
        for idx, port in enumerate(self.ports):
            self.port_list.insert(idx, port)

        self.frame7 = tk.Frame(self.bot_frame, background="#888888")
        self.frame7.pack(fill=tk.BOTH, side=tk.LEFT, expand=True)
        self.f7_label = tk.Label(self.frame7, text="Controls")
        self.f7_label.pack(fill=tk.X)

        self.start_button = tk.Button(self.frame7, text="Start", command=self._start)
        self.start_button.pack(side=tk.TOP, expand=True)

        self.stop_button = tk.Button(self.frame7, text="Stop", command=self._stop)
        self.stop_button.pack(side=tk.TOP, expand=True)

        self.init_reg_tree()

    def init_reg_tree(self):
        self.reg_label = tk.Label(self.left_frame, text="Registers")
        self.reg_label.pack(fill=tk.X)
        self.reg_tree = ttk.Treeview(self.left_frame, columns=["value", "previous"])
        self.reg_tree.pack(fill=tk.BOTH, side=tk.LEFT, expand=True)

        self.reg_tree.heading("#0", text="Registers")
        self.reg_tree.heading("value", text="Value")
        self.reg_tree.heading("previous", text="Previous")

        self.reg_tree.column("#0", width=50, minwidth=50)
        self.reg_tree.column("value", width=50, minwidth=50)
        self.reg_tree.column("previous", width=50, minwidth=50)

        self.reg_tree.insert("", "end", "a", text="A:", values=[0, 0])
        self.reg_tree.insert("", "end", "p", text="P:", values=[0, 0])
        self.reg_tree.insert("", "end", "pc", text="PC:", values=[0, 0])
        self.reg_tree.insert("", "end", "s", text="S:", values=[0, 0])
        self.reg_tree.insert("", "end", "x", text="X:", values=[0, 0])
        self.reg_tree.insert("", "end", "y", text="Y:", values=[0, 0])

    def select_port(self):
        try:
            return self.ports[self.port_list.curselection()[0]].device
        except IndexError:
            return None

    def _start(self):
        self.started = True

    def _stop(self):
        self.started = False

    def update_reg_tree(self, iid, values):
        if self.reg_tree.exists(iid):
            self.reg_tree.item(iid, values=values)

    def dump(self):
        pass
