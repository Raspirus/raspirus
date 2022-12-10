import tkinter as tk
from raspirus.frontend.popups.DoubleButtonDialog import DoubleButtonDialog
# For colors and fonts
from raspirus.frontend.utility import \
    BACKGROUND_COLOR, SUBTITLE_FONT, \
    FAILURE_COLOR, NORMAL_TEXT_FONT, TEXT_COLOR


class VirusPage(tk.Frame):
    title_label: tk.Label
    confirm_btn: tk.Button
    virus_list: tk.Listbox
    page_scroller: tk.Scrollbar

    controller = None

    def __init__(self, parent, controller):
        tk.Frame.__init__(self, parent, bg=BACKGROUND_COLOR)

        self.controller = controller

        self.title_label = tk.Label(self, text="VIRUS FOUND", font=SUBTITLE_FONT,
                                    fg=FAILURE_COLOR, bg=BACKGROUND_COLOR)
        self.title_label.place(x=190, y=50, width=415, height=60)

        self.page_scroller = tk.Scrollbar(self, orient="vertical")
        self.page_scroller.place(x=100, y=160, width=600, height=150)

        self.virus_list = tk.Listbox(self, font=NORMAL_TEXT_FONT,
                                     fg=BACKGROUND_COLOR, bg=TEXT_COLOR)
        self.virus_list.place(x=100, y=160, width=600, height=150)

        self.confirm_btn = tk.Button(self, text="CONFIRM", font=NORMAL_TEXT_FONT,
                                     fg=BACKGROUND_COLOR, bg=FAILURE_COLOR)
        self.confirm_btn.config(command=lambda: controller.show_frame(controller.pages[0]))
        self.confirm_btn.place(x=315, y=375, width=170, height=50)

    def add_viruses(self, virus_arr):
        # For each element in the given array, create a component:
        # Component has a label with the path to the virus file and a combobox
        # the combobox contains possible actions
        # This component also has a red border around it
        # Then add these components to a scrollbar frame
        count = 1
        for virus in virus_arr:
            self.virus_list.insert(count, str(virus.path))
            count += 1

    def confirm_actions(self):
        dialog_message = "Your selected actions might be unrecoverable, are you sure you want to continue?"
        dialog = DoubleButtonDialog(title="Confirm actions", parent=self, message=dialog_message)
        dialog.tkraise()

    def confirm_btn_func(self):
        # Needed for the dialog
        self.controller.show_frame(self.controller.pages[0])  # type: ignore [union-attr]

    def deny_btn_func(self):
        # Needed for the dialog
        pass
