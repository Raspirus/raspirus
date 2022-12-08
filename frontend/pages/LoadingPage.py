import tkinter as tk
from tkinter import ttk
from Raspirus.frontend.utility import *  # For colors and fonts


class LoadingPage(tk.Frame):
    progress_bar: ttk.Progressbar
    title_label: tk.Label
    scanned_label: tk.Label
    vfound_label: tk.Label
    quit_btn: tk.Button

    def __init__(self, parent, controller):
        tk.Frame.__init__(self, parent, bg=BACKGROUND_COLOR)

        self.title_label = tk.Label(self, text="Scanning... Please wait", font=SUBTITLE_FONT,
                                    fg=SECONDARY_COLOR, bg=BACKGROUND_COLOR)
        self.title_label.place(x=55, y=115, width=670, height=125)

        pstyle = ttk.Style()  # ProgressBar doesn't support coloring, so we need to create an external style and
        # apply it
        # ISSUE: Color defined here is not being used, only if you add pstyle.theme('clam')
        # Theme list: https://wiki.tcl-lang.org/page/List+of+ttk+Themes
        pstyle.configure("primary.Horizontal.TProgressbar", foreground=BACKGROUND_COLOR, background=PRIMARY_COLOR)
        self.progress_bar = ttk.Progressbar(self, orient='horizontal', mode='determinate',
                                            style="primary.Horizontal.TProgressbar",
                                            maximum=100, value=65)
        self.progress_bar.place(x=50, y=225, width=700, height=45)

        self.scanned_label = tk.Label(self, text="Scanned 12.345 files of 24.478 total", font=NORMAL_TEXT_FONT,
                                      fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.scanned_label.place(x=50, y=275, width=280, height=30)

        self.vfound_label = tk.Label(self, text="Virus found: 0", font=NORMAL_TEXT_FONT,
                                     fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.vfound_label.place(x=50, y=315, width=280, height=30)

        # TODO: Set a better Icon here, bigger!
        self.abort_icon = tk.PhotoImage(file="frontend/images/icons/cancel_sign.png")
        self.quit_btn = tk.Button(self, fg=BACKGROUND_COLOR, bg=FAILURE_COLOR,
                                  image=self.abort_icon)
        self.quit_btn.config(command=lambda: controller.show_frame(controller.pages[0]))
        self.quit_btn.place(x=375, y=375, width=50, height=50)

    def setLoadingStatus(self, status: int):
        if 0 <= status <= 100:
            self.progress_bar.config(value=status)
