import tkinter as tk
from tkinter import ttk

from raspirus.frontend.helpers.ImageLabel import ImageLabel
from raspirus.frontend.popups.DoubleButtonDialog import DoubleButtonDialog
# For colors and fonts
from raspirus.frontend.utility import \
    BACKGROUND_COLOR, SECONDARY_COLOR, SUBTITLE_FONT, FAILURE_COLOR


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
        tk.Frame.__init__(self, parent, bg='#1b1d25')
        self.controller = controller

        self.anim_label = ImageLabel(self, borderwidth=0, bg='#1b1d25')
        self.anim_label.pack(expand=True, fill=tk.BOTH, anchor=tk.CENTER)
        self.anim_label.load('frontend/images/loading_animation.gif')

        self.title_label = tk.Label(self, text="Scanning... Please wait", font=SUBTITLE_FONT,
                                    fg=SECONDARY_COLOR, bg='#1b1d25')
        self.title_label.place(x=55, y=50, width=670, height=125)

        self.abort_icon = tk.PhotoImage(file="frontend/images/icons/cancel_sign.png")
        self.quit_btn = tk.Button(self, fg=BACKGROUND_COLOR, bg=FAILURE_COLOR,
                                  image=self.abort_icon)
        self.quit_btn.config(command=lambda: self.confirm_quit())
        self.quit_btn.place(x=375, y=375, width=50, height=50)

    def confirm_quit(self):
        dialog_message = "Warning! The scanner hasn't finished yet, are you sure you want to terminate it?"
        dialog = DoubleButtonDialog(title="Stop scanner", parent=self, message=dialog_message)
        dialog.tkraise()

    def confirm_btn_func(self):
        # Needed for the dialog
        self.controller.show_frame(self.controller.pages[0])  # type: ignore [union-attr]

    def deny_btn_func(self):
        # Needed for the dialog
        pass
