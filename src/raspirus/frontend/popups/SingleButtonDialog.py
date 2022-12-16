import tkinter as tk
from typing import Literal
# For colors and fonts
from raspirus.frontend.utility import \
    BACKGROUND_COLOR, SUCCESS_COLOR, FAILURE_COLOR, \
    WARNING_COLOR, SUBTITLE_FONT, NORMAL_TEXT_FONT, \
    TEXT_COLOR, BUTTON_TEXT_FONT


class SingleButtonDialog(tk.Toplevel):
    mode: str
    parent = None
    MODE = Literal["warning", "error", "confirm"]
    title_label: tk.Label
    msg_label: tk.Label
    confirm_btn: tk.Button

    def __init__(self, parent, title, message, mode: MODE):
        tk.Toplevel.__init__(self, bg=BACKGROUND_COLOR)
        self.wm_title(title)
        self.parent = parent
        self.mode = mode

        self.wm_resizable(False, False)
        self.overrideredirect(True)  # Borderless
        self.center()

        self.wm_geometry("500x250")

        # This is watching the window manager close button
        # and uses the same callback function as the other buttons
        # (you can use which ever you want, BUT REMEMBER TO ENABLE
        # THE PARENT WINDOW AGAIN)
        self.protocol("WM_DELETE_WINDOW", self.close_dialog)

        # Decide which color to use depending on the mode
        color = SUCCESS_COLOR
        if mode == "warning":
            color = WARNING_COLOR
        elif mode == "error":
            color = FAILURE_COLOR

        # Forces all interactions to go through this modal dialog:
        self.grab_set()

        self.title_label = tk.Label(self, text=title, font=SUBTITLE_FONT,
                                    fg=BACKGROUND_COLOR, bg=color)
        self.title_label.place(x=0, y=0, width=500, height=70)

        self.msg_label = tk.Label(self, text=message, font=NORMAL_TEXT_FONT,
                                  fg=TEXT_COLOR, bg=BACKGROUND_COLOR,
                                  wraplength=480, justify="center")
        self.msg_label.place(x=15, y=80, width=480, height=90)

        self.confirm_btn = tk.Button(self, text="CONFIRM", font=BUTTON_TEXT_FONT,
                                     fg=BACKGROUND_COLOR, bg=color)
        self.confirm_btn.config(command=lambda: self.close_dialog())
        self.confirm_btn.place(x=150, y=185, width=175, height=40)

    def close_dialog(self):

        self.destroy()

    def center(self):
        """
        centers a tkinter window
        :param self: the main window or Toplevel window to center
        """
        self.update_idletasks()
        width = self.winfo_width()
        frm_width = self.winfo_rootx() - self.winfo_x()
        win_width = width + 2 * frm_width
        height = self.winfo_height()
        titlebar_height = self.winfo_rooty() - self.winfo_y()
        win_height = height + titlebar_height + frm_width
        x = self.winfo_screenwidth() // 2 - win_width // 2
        y = self.winfo_screenheight() // 2 - win_height // 2
        self.geometry(f'{width}x{height}+{x}+{y}')
        self.deiconify()
