import tkinter as tk
from raspirus.frontend.utility import *  # For colors and fonts
from PIL import Image, ImageTk


class ClearPage(tk.Frame):
    title_label: tk.Label
    image_container: tk.Canvas
    confirm_btn: tk.Button

    def __init__(self, parent, controller):
        tk.Frame.__init__(self, parent, bg=BACKGROUND_COLOR)

        self.title_label = tk.Label(self, text="NO VIRUS FOUND", font=SUBTITLE_FONT,
                                    fg=SUCCESS_COLOR, bg=BACKGROUND_COLOR)
        self.title_label.place(x=190, y=50, width=450, height=60)

        # WARNING! Image has the size 160x160 and unfortunately can't be resized with Tkinter
        self.image_loader = Image.open("frontend/images/success_image.png")
        self.success_image = ImageTk.PhotoImage(self.image_loader)

        self.image_container = tk.Canvas(self, width=160, height=160, bg=BACKGROUND_COLOR,
                                         highlightthickness=0)
        self.image_container.place(x=320, y=160)
        self.image_container.create_image(0, 0, anchor='nw', image=self.success_image)

        self.confirm_btn = tk.Button(self, text="CONFIRM", font=NORMAL_TEXT_FONT,
                                     fg=BACKGROUND_COLOR, bg=SUCCESS_COLOR)
        self.confirm_btn.config(command=lambda: controller.show_frame(controller.pages[0]))
        self.confirm_btn.place(x=315, y=375, width=170, height=50)

