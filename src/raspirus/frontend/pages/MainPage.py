import tkinter as tk
from tkinter import ttk
import os
import platform
from raspirus.frontend.popups.SingleButtonDialog import SingleButtonDialog
# For colors and fonts
from raspirus.frontend.utility import \
    BACKGROUND_COLOR, TITLE_FONT, PRIMARY_COLOR, \
    NORMAL_TEXT_FONT, BUTTON_TEXT_FONT, TEXT_COLOR, \
    SMALL_BUTTON_TEXT_FONT, SECONDARY_COLOR


class MainPage(tk.Frame):
    title_label: tk.Label
    drive_selector: ttk.Combobox
    start_btn: tk.Button
    info_btn: tk.Button
    settings_btn: tk.Button

    def __init__(self, parent, controller):
        tk.Frame.__init__(self, parent, bg=BACKGROUND_COLOR)

        self.title_label = tk.Label(self, text="RASPIRUS",
                                    font=TITLE_FONT, fg=PRIMARY_COLOR, bg=BACKGROUND_COLOR)
        self.title_label.place(x=140, y=60, width=510, height=120)

        # highlightbackground / highlightcolor sets a border to the component
        self.drive_selector = ttk.Combobox(self, font=NORMAL_TEXT_FONT, state='readonly')
        self.drive_selector.place(x=90, y=215, width=620, height=48)
        self.load_drive_list()

        self.start_btn = tk.Button(self, text="START", font=BUTTON_TEXT_FONT,
                                   fg=BACKGROUND_COLOR, bg=PRIMARY_COLOR)
        self.start_btn.config(command=lambda: self.start_scanner(controller=controller))
        self.start_btn.place(x=185, y=315, width=170, height=50)

        self.info_btn = tk.Button(self, text="INFO", font=BUTTON_TEXT_FONT,
                                  fg=BACKGROUND_COLOR, bg=TEXT_COLOR)
        self.info_btn.config(command=lambda: controller.show_frame(controller.pages[3]))
        self.info_btn.place(x=420, y=315, width=170, height=50)

        self.settings_btn = tk.Button(self, text="SETTINGS", font=SMALL_BUTTON_TEXT_FONT,
                                      fg=SECONDARY_COLOR, bg=TEXT_COLOR)
        self.settings_btn.config(command=lambda: controller.show_frame(controller.pages[1]))
        self.settings_btn.place(x=670, y=15, width=110, height=40)

    def load_drive_list(self):
        # Windows
        # Find all connected USB drives
        drives = []
        # Check the platform
        if platform.system() == "Windows":
            for letter in "ABCDEFGHIJKLMNOPQRSTUVWXYZ":
                path = f"{letter}:\\"
                if os.path.isdir(path):
                    drives.append(path)
            return drives
        else:
            with open("/proc/mounts", "r") as f:
                for line in f:
                    if line.startswith("/dev/sd"):
                        # Extract the drive path
                        drive = line.split()[1]
                        drives.append(drive)

        # For testing purpose only:
        drives.append("C:/Users/benbe/Documents/Coding/PyProjects/MaturaProject/tests/files")

        self.drive_selector["values"] = drives
        self.drive_selector.current(0)

    def start_scanner(self, controller):
        # Checks if the given string is empty
        if len(self.drive_selector.get()) <= 0 or len(str(self.drive_selector.get()).strip()) <= 0:
            no_drive_message = "Before starting the scanner you need to specify which " \
                               "harddrive or USB you want to scan by selecting " \
                               "it from the dropdown menu"
            dialog = SingleButtonDialog(title="No Drive", parent=self,
                                        message=no_drive_message, mode="error")
            dialog.tkraise()
        else:
            controller.scanning_path = self.drive_selector.get()
            controller.start_scanner()
