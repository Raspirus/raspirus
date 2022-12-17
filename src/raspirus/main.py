"""This module starts other frontend pages and their backend

Classes: Windows
"""

# Importing the tkinter module
import tkinter as tk
import ctypes
import threading

# Frontend:
from raspirus.frontend.pages.ClearPage import ClearPage
from raspirus.frontend.pages.InfoPage import InfoPage
from raspirus.frontend.pages.LoadingPage import LoadingPage
from raspirus.frontend.pages.MainPage import MainPage
from raspirus.frontend.pages.SettingsLogPage import SettingsLogPage
from raspirus.frontend.pages.SettingsPage import SettingsPage
from raspirus.frontend.pages.VirusPage import VirusPage

# Backend:
from raspirus.backend.file_scanner_module import FileScanner
from raspirus.backend.database_api import HashAPI

# Sets a higher resolution on Tkinter frames
ctypes.windll.shcore.SetProcessDpiAwareness(1)


class Windows(tk.Tk):
    """This class contains all other pages of the application and can call them

    Methods:
        __init__(self)
        show_frame(self, cont)
    """
    # Items have a fixed order! -> Contains all pages as a reference
    pages = (MainPage, SettingsPage, LoadingPage, InfoPage, VirusPage, ClearPage, SettingsLogPage)

    # App properties: name, version, creator, license, contact
    properties = ["Raspirus", "v1.0.0", "Benjamin Demetz", "GPL", None]

    # Logs properties:
    log_file_location = "../notes.txt"

    # Scanner properties
    scanning_path = ""
    database_path = "backend/database/signatures.db"
    file_scanner: FileScanner
    hash_updater: HashAPI

    def __init__(self):
        """ Initializes the class """
        # Adding a title to the window
        tk.Tk.__init__(self)
        self.wm_title("Raspirus")
        self.wm_geometry("800x480")
        self.wm_resizable(width=False, height=False)

        # creating a frame and assigning it to container
        container = tk.Frame(self)
        # specifying the region where the frame is packed in root
        container.pack(side="top", fill="both", expand=True)

        # configuring the location of the container using grid
        container.grid_rowconfigure(0, weight=1)
        container.grid_columnconfigure(0, weight=1)

        # We will now create a dictionary of frames
        self.frames = {}
        for Frame in self.pages:
            frame = Frame(container, self)

            if isinstance(frame, InfoPage):
                frame.setProperties(self.properties)

            # the windows class acts as the root window for the frames.
            self.frames[Frame] = frame
            frame.grid(row=0, column=0, sticky="nsew")

        # Using a method to switch frames
        self.show_frame(MainPage)
        # self.show_frame(LoadingPage)

    def show_frame(self, cont):
        """This method opens a new frame by giving it the ID
           of the frame contained in the frames variable

        Arguments:
            cont -> Index of the frame
        """
        frame = self.frames[cont]
        # raises the current frame to the top
        frame.tkraise()

    def thread_helper(self):
        self.file_scanner = FileScanner(path=self.scanning_path, db_location=self.database_path)
        self.file_scanner.start_scanner()
        self.evaluate_scanner()

    def start_scanner(self):
        self.show_frame(LoadingPage)
        t = threading.Thread(target=self.thread_helper)
        t.start()

    def evaluate_scanner(self):
        if len(self.file_scanner.dirty_files) > 0:
            virus_page = self.frames[VirusPage]
            virus_page.add_viruses(self.file_scanner.dirty_files)
            self.show_frame(VirusPage)
        else:
            self.show_frame(ClearPage)

    def start_hash_updates(self):
        self.hash_updater = HashAPI(self.database_path)
        self.hash_updater.update_db()


if __name__ == "__main__":
    app = Windows()
    app.mainloop()
