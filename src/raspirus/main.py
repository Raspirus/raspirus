"""This module starts other frontend pages and their backend

Classes: Windows
"""

# Importing the tkinter module
import tkinter as tk
import ctypes

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
from raspirus.backend.hash_api_module import HashAPI

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
    scanner: FileScanner
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
        # self.show_frame(MainPage)
        self.show_frame(LoadingPage)

    def show_frame(self, cont):
        """This method opens a new frame by giving it the ID
           of the frame contained in the frames variable

        Arguments:
            cont -> Index of the frame
        """
        frame = self.frames[cont]
        # raises the current frame to the top
        frame.tkraise()

    def start_scanner(self):
        loading_page = self.frames[LoadingPage]
        # loading_page.print_tests()
        # TODO: Make this a thread -> Else blocks the button
        self.scanner = FileScanner(path=self.scanning_path, db_location=self.database_path)
        self.scanner.start_scanner()
        loading_page.set_maximum(0)
        self.show_frame(LoadingPage)
        # self.scanner.start_scanner() # Continue from here once this is done
        # self.evaluate_scanner()

    def evaluate_scanner(self):
        if len(self.scanner.dirty_files) > 0:
            virus_page = self.frames[VirusPage]
            virus_page.add_viruses(self.scanner.dirty_files)
            self.show_frame(VirusPage)
        else:
            self.show_frame(ClearPage)

    def start_hash_updates(self):
        self.hash_updater = HashAPI(self.signature_lists_path, self.signature_path)
        self.hash_updater.update_bighash()


if __name__ == "__main__":
    app = Windows()
    app.mainloop()
