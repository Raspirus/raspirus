import tkinter as tk
from raspirus.frontend.utility import *  # For colors and fonts


class InfoPage(tk.Frame):
    home_btn: tk.Button
    title_label: tk.Label
    # r-prefix: right aligned text
    # l-prefix: left aligned text
    lname_label: tk.Label
    rname_label: tk.Label
    lversion_label: tk.Label
    rversion_label: tk.Label
    lcreator_label: tk.Label
    rcreator_label: tk.Label
    llicense_label: tk.Label
    rlicense_label: tk.Label
    lcontact_label: tk.Label
    rcontact_label: tk.Label

    def __init__(self, parent, controller):
        tk.Frame.__init__(self, parent, bg=BACKGROUND_COLOR)

        # Button to return to the main page
        self.arrow_icon = tk.PhotoImage(file="frontend/images/icons/back_arrow.png")
        self.home_btn = tk.Button(self, text="HOME", font=SMALL_BUTTON_TEXT_FONT,
                                  image=self.arrow_icon, compound=tk.LEFT, padx=10,
                                  fg=BACKGROUND_COLOR, bg=GREY_COLOR)
        self.home_btn.config(command=lambda: controller.show_frame(controller.pages[0]))
        self.home_btn.place(x=20, y=30, width=110, height=30)

        self.title_label = tk.Label(self, text="APP INFO", font=SUBTITLE_FONT,
                                    fg=SECONDARY_COLOR, bg=BACKGROUND_COLOR)
        self.title_label.place(x=190, y=50, width=415, height=60)

        self.lname_label = tk.Label(self, text="Name:", font=NORMAL_TEXT_FONT,
                                    fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.lname_label.place(x=45, y=170, width=360, height=50)

        self.rname_label = tk.Label(self, text="Raspirus", font=NORMAL_TEXT_FONT,
                                    fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.rname_label.place(x=405, y=170, width=360, height=50)

        self.lversion_label = tk.Label(self, text="Version:", font=NORMAL_TEXT_FONT,
                                       fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.lversion_label.place(x=45, y=220, width=360, height=50)

        self.rversion_label = tk.Label(self, text="v1.0.0", font=NORMAL_TEXT_FONT,
                                       fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.rversion_label.place(x=405, y=220, width=360, height=50)

        self.lcreator_label = tk.Label(self, text="Creator:", font=NORMAL_TEXT_FONT,
                                       fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.lcreator_label.place(x=45, y=270, width=360, height=50)

        self.rcreator_label = tk.Label(self, text="Benjamin Demetz", font=NORMAL_TEXT_FONT,
                                       fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.rcreator_label.place(x=405, y=270, width=360, height=50)

        self.llicense_label = tk.Label(self, text="License", font=NORMAL_TEXT_FONT,
                                       fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.llicense_label.place(x=45, y=320, width=360, height=50)

        self.rlicense_label = tk.Label(self, text="Alperia AG", font=NORMAL_TEXT_FONT,
                                       fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.rlicense_label.place(x=405, y=320, width=360, height=50)

        self.lcontact_label = tk.Label(self, text="Contact", font=NORMAL_TEXT_FONT,
                                       fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.lcontact_label.place(x=45, y=370, width=360, height=50)

        self.rcontact_label = tk.Label(self, text="demetzbenjamin23@gmail.com", font=NORMAL_TEXT_FONT,
                                       fg=TEXT_COLOR, bg=BACKGROUND_COLOR, anchor='w')
        self.rcontact_label.place(x=405, y=370, width=360, height=50)

    def setProperties(self, properties: [4]):
        """ A function to fill information about the app
        If None is given, the default value is used

        Arguments:
            properties -> An array containing 5 string elements

        """

        if properties[0] is not None:
            self.rname_label.config(text=properties[0])

        if properties[1] is not None:
            self.rversion_label.config(text=properties[1])

        if properties[2] is not None:
            self.rcreator_label.config(text=properties[2])

        if properties[3] is not None:
            self.rlicense_label.config(text=properties[3])

        if properties[4] is not None:
            self.rcontact_label.config(text=properties[4])
