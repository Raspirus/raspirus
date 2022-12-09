import tkinter as tk
from tkinter import ttk
from raspirus.frontend.popups.SingleButtonDialog import SingleButtonDialog
from raspirus.frontend.utility import *  # For colors and fonts


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
        # Linux options: https://stackoverflow.com/a/8265634
        # Windows: https://stackoverflow.com/a/8110666
        test_list = [
            "C:/Users/benbe/Documents/Coding/MaturaProject/Raspirus/testing/files",
            "Some more teststs",
            " With some space front",
            "Short",
            " Very very very very very vfery hags hasgwqgdi iqdidnob iqbiq LONG",
            "      "
        ]
        self.drive_selector["values"] = test_list
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

