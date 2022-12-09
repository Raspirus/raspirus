import tkinter as tk
from tkinter import ttk
from raspirus.frontend.popups.DoubleButtonDialog import DoubleButtonDialog
from raspirus.frontend.utility import *  # For colors and fonts


class LoadingPage(tk.Frame):
    progress_bar: ttk.Progressbar
    title_label: tk.Label
    scanned_label: tk.Label
    vfound_label: tk.Label
    quit_btn: tk.Button

    controller = None

    max_text: int
    current_scan_text = 0
    current_virus_text = 0

    def __init__(self, parent, controller):
        tk.Frame.__init__(self, parent, bg=BACKGROUND_COLOR)

        self.controller = controller

        self.title_label = tk.Label(self, text="Scanning... Please wait", font=SUBTITLE_FONT,
                                    fg=SECONDARY_COLOR, bg=BACKGROUND_COLOR)
        self.title_label.place(x=55, y=115, width=670, height=125)

        pstyle = ttk.Style()
        # TODO: ProgressBar doesn't support coloring, so we need to create an external style and apply it
        # ISSUE: Color defined here is not being used, only if you add pstyle.theme('clam')
        # Theme list: https://wiki.tcl-lang.org/page/List+of+ttk+Themes
        pstyle.configure("primary.Horizontal.TProgressbar", foreground=BACKGROUND_COLOR, background=PRIMARY_COLOR)
        self.progress_bar = ttk.Progressbar(self, orient='horizontal', mode='determinate',
                                            style="primary.Horizontal.TProgressbar",
                                            maximum=100, value=65)
        self.progress_bar.place(x=50, y=225, width=700, height=45)

        self.scanned_label = tk.Label(self, text="Scanned 0 files of X total", font=NORMAL_TEXT_FONT,
                                      fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.scanned_label.place(x=50, y=275, width=280, height=30)

        self.vfound_label = tk.Label(self, text="Virus found: 0", font=NORMAL_TEXT_FONT,
                                     fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.vfound_label.place(x=50, y=315, width=280, height=30)

        # TODO: Set a better Icon here, bigger!
        self.abort_icon = tk.PhotoImage(file="frontend/images/icons/cancel_sign.png")
        self.quit_btn = tk.Button(self, fg=BACKGROUND_COLOR, bg=FAILURE_COLOR,
                                  image=self.abort_icon)
        self.quit_btn.config(command=lambda: self.confirm_quit())
        self.quit_btn.place(x=375, y=375, width=50, height=50)

    def set_loading_status(self, status: int):
        if 0 <= status <= 100:
            self.progress_bar.config(value=status)

    def set_maximum(self, maximum: int):
        self.max_text = maximum
        scanned_text = "Scanned " + str(self.current_scan_text) + " files of " + str(self.max_text) + " total"
        self.scanned_label.config(text=scanned_text)

    def increase_scanned(self):
        self.current_scan_text += 1
        scanned_text = "Scanned " + str(self.current_scan_text) + " files of "

        if self.max_text is not None:
            scanned_text += str(self.max_text)
        else:
            scanned_text += "Undefined"
        scanned_text += " total"

        self.scanned_label.config(text=scanned_text)

    def increase_virus(self):
        self.current_virus_text += 1
        virus_text = "Virus found: " + str(self.current_virus_text)
        self.vfound_label.config(text=virus_text)

    def confirm_quit(self):
        dialog_message = "Warning! The scanner hasn't finished yet, are you sure you want to terminate it?"
        dialog = DoubleButtonDialog(title="Stop scanner", parent=self, message=dialog_message)
        dialog.tkraise()

    def confirm_btn_func(self):
        # Needed for the dialog
        self.controller.show_frame(self.controller.pages[0])

    def deny_btn_func(self):
        # Needed for the dialog
        pass

