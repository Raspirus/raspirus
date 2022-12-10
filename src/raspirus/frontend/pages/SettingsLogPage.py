import tkinter as tk
# For colors and fonts
from raspirus.frontend.utility import BACKGROUND_COLOR, SMALL_BUTTON_TEXT_FONT, GREY_COLOR


class SettingsLogPage(tk.Frame):
    home_btn: tk.Button
    logs_textbox: tk.Text

    def __init__(self, parent, controller):
        tk.Frame.__init__(self, parent, bg=BACKGROUND_COLOR)

        # Button to return to the main page
        self.arrow_icon = tk.PhotoImage(file="frontend/images/icons/back_arrow.png")
        self.home_btn = tk.Button(self, text="BACK", font=SMALL_BUTTON_TEXT_FONT,
                                  image=self.arrow_icon, compound=tk.LEFT, padx=10,
                                  fg=BACKGROUND_COLOR, bg=GREY_COLOR)
        self.home_btn.config(command=lambda: controller.show_frame(controller.pages[1]))
        self.home_btn.place(x=20, y=30, width=110, height=30)

        self.logs_textbox = tk.Text(self)
        self.logs_textbox.place(x=25, y=80, width=750, height=370)

        self.set_log_text(controller)

    def set_log_text(self, controller):
        # First we need to delete everything from the textbox
        self.logs_textbox.delete('1.0', tk.END)

        content = "No logs found"

        # Then we extract the text from the given logs file
        try:
            with open(controller.log_file_location) as f:
                content = ""
                line_content = f.readlines()
                for line in line_content:
                    content += line

        except Exception as err:
            print("Logs not found: " + str(err))

        # Then we can insert new text at position 0
        self.logs_textbox.insert('1.0', content)
