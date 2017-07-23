#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]

use imgui_sys;
//use std::marker::PhantomData;
//use std::ptr;

use imgui::{Ui, ImGuiSetCond_FirstUseEver, ImVec2, ImGuiWindowFlags};

pub struct HexEditor {
    open: bool,
    allow_edit: bool,
    rows: i32,
    data_edit_addr: i32,
    data_edit_take_focus: bool,
    data_input: Vec<u8>,
    addr_input: Vec<u8>
}
impl Default for HexEditor {
    fn default() -> Self {
        HexEditor {
            open: true,
            allow_edit: true,
            rows: 16,
            data_edit_addr: -1,
            data_edit_take_focus: false,
            data_input: vec![32],
            addr_input: vec![32],
        }
    }
}
impl HexEditor {
    pub fn render<'a>(&self, ui: &Ui<'a>, title: &str, mut data: &Vec<u8>) {
        ui.window(im_str!("{}", title))
        .size((300.0, 300.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            unsafe {
                imgui_sys::igBeginChild(im_str!("##scrolling").as_ptr(), ImVec2::new(0.0, -imgui_sys::igGetItemsLineHeightWithSpacing()), true, ImGuiWindowFlags::empty());

                imgui_sys::igPushStyleVarVec(imgui_sys::ImGuiStyleVar::FramePadding, ImVec2::new(0.0, 0.0));
                imgui_sys::igPushStyleVarVec(imgui_sys::ImGuiStyleVar::ItemSpacing, ImVec2::new(0.0, 0.0));

                let addr_digits_count = 4;

                let mut ig_vec = ImVec2::new(0.0, 0.0);
                imgui_sys::igCalcTextSize(&mut ig_vec, im_str!("FF").as_ptr(), im_str!("").as_ptr(), false, 9999.0);
                let glyph_width = ig_vec.x;
                let cell_width = glyph_width * 3.; // "FF " we include trailing space in the width to easily catch clicks everywhere

                let line_height = imgui_sys::igGetTextLineHeight();
                let line_total_count = ((data.len() as i32 + self.rows-1) / self.rows) as i32;

//                let clipper = imgui_sys::ImGuiListClipper::default(line_total_count, line_height);
//
//                let visible_start_addr = clipper.display_start * self.rows;
//                let visible_end_addr = clipper.display_end * self.rows;

                imgui_sys::igPopStyleVar(2);
                imgui_sys::igEndChild();
            }
        })
    }
}
/*

            float line_height = ImGui::GetTextLineHeight();
            int line_total_count = (int)((mem_size + Rows-1) / Rows);
            ImGuiListClipper clipper(line_total_count, line_height);
            int visible_start_addr = clipper.DisplayStart * Rows;
            int visible_end_addr = clipper.DisplayEnd * Rows;

            bool data_next = false;

            if (!AllowEdits || DataEditingAddr >= mem_size)
                DataEditingAddr = -1;

            int data_editing_addr_backup = DataEditingAddr;
            if (DataEditingAddr != -1)
            {
                if (ImGui::IsKeyPressed(ImGui::GetKeyIndex(ImGuiKey_UpArrow)) && DataEditingAddr >= Rows)                   { DataEditingAddr -= Rows; DataEditingTakeFocus = true; }
                else if (ImGui::IsKeyPressed(ImGui::GetKeyIndex(ImGuiKey_DownArrow)) && DataEditingAddr < mem_size - Rows)  { DataEditingAddr += Rows; DataEditingTakeFocus = true; }
                else if (ImGui::IsKeyPressed(ImGui::GetKeyIndex(ImGuiKey_LeftArrow)) && DataEditingAddr > 0)                { DataEditingAddr -= 1; DataEditingTakeFocus = true; }
                else if (ImGui::IsKeyPressed(ImGui::GetKeyIndex(ImGuiKey_RightArrow)) && DataEditingAddr < mem_size - 1)    { DataEditingAddr += 1; DataEditingTakeFocus = true; }
            }
            if ((DataEditingAddr / Rows) != (data_editing_addr_backup / Rows))
            {
                // Track cursor movements
                float scroll_offset = ((DataEditingAddr / Rows) - (data_editing_addr_backup / Rows)) * line_height;
                bool scroll_desired = (scroll_offset < 0.0f && DataEditingAddr < visible_start_addr + Rows*2) || (scroll_offset > 0.0f && DataEditingAddr > visible_end_addr - Rows*2);
                if (scroll_desired)
                    ImGui::SetScrollY(ImGui::GetScrollY() + scroll_offset);
            }

            bool draw_separator = true;
            for (int line_i = clipper.DisplayStart; line_i < clipper.DisplayEnd; line_i++) // display only visible items
            {
                int addr = line_i * Rows;
                ImGui::Text("%0*X: ", addr_digits_count, base_display_addr+addr);
                ImGui::SameLine();

                // Draw Hexadecimal
                float line_start_x = ImGui::GetCursorPosX();
                for (int n = 0; n < Rows && addr < mem_size; n++, addr++)
                {
                    ImGui::SameLine(line_start_x + cell_width * n);

                    if (DataEditingAddr == addr)
                    {
                        // Display text input on current byte
                        ImGui::PushID(addr);
                        struct FuncHolder
                        {
                            static int Callback(ImGuiTextEditCallbackData* data)
                            {
                                int* p_cursor_pos = (int*)data->UserData;
                                if (!data->HasSelection())
                                    *p_cursor_pos = data->CursorPos;
                                return 0;
                            }
                        };
                        int cursor_pos = -1;
                        bool data_write = false;
                        if (DataEditingTakeFocus)
                        {
                            ImGui::SetKeyboardFocusHere();
                            sprintf(AddrInput, "%0*X", addr_digits_count, base_display_addr+addr);
                            sprintf(DataInput, "%02X", mem_data[addr]);
                        }
                        ImGui::PushItemWidth(ImGui::CalcTextSize("FF").x);
                        ImGuiInputTextFlags flags = ImGuiInputTextFlags_CharsHexadecimal|ImGuiInputTextFlags_EnterReturnsTrue|ImGuiInputTextFlags_AutoSelectAll|ImGuiInputTextFlags_NoHorizontalScroll|ImGuiInputTextFlags_AlwaysInsertMode|ImGuiInputTextFlags_CallbackAlways;
                        if (ImGui::InputText("##data", DataInput, 32, flags, FuncHolder::Callback, &cursor_pos))
                            data_write = data_next = true;
                        else if (!DataEditingTakeFocus && !ImGui::IsItemActive())
                            DataEditingAddr = -1;
                        DataEditingTakeFocus = false;
                        ImGui::PopItemWidth();
                        if (cursor_pos >= 2)
                            data_write = data_next = true;
                        if (data_write)
                        {
                            int data;
                            if (sscanf(DataInput, "%X", &data) == 1)
                                mem_data[addr] = (unsigned char)data;
                        }
                        ImGui::PopID();
                    }
                    else
                    {
                        ImGui::Text("%02X ", mem_data[addr]);
                        if (AllowEdits && ImGui::IsItemHovered() && ImGui::IsMouseClicked(0))
                        {
                            DataEditingTakeFocus = true;
                            DataEditingAddr = addr;
                        }
                    }
                }

                ImGui::SameLine(line_start_x + cell_width * Rows + glyph_width * 2);

                if (draw_separator)
                {
                    ImVec2 screen_pos = ImGui::GetCursorScreenPos();
                    ImGui::GetWindowDrawList()->AddLine(ImVec2(screen_pos.x - glyph_width, screen_pos.y - 9999), ImVec2(screen_pos.x - glyph_width, screen_pos.y + 9999), ImColor(ImGui::GetStyle().Colors[ImGuiCol_Border]));
                    draw_separator = false;
                }

                // Draw ASCII values
                addr = line_i * Rows;
                for (int n = 0; n < Rows && addr < mem_size; n++, addr++)
                {
                    if (n > 0) ImGui::SameLine();
                    int c = mem_data[addr];
                    ImGui::Text("%c", (c >= 32 && c < 128) ? c : '.');
                }
            }
            clipper.End();
            ImGui::PopStyleVar(2);

            ImGui::EndChild();

            if (data_next && DataEditingAddr < mem_size)
            {
                DataEditingAddr = DataEditingAddr + 1;
                DataEditingTakeFocus = true;
            }

            ImGui::Separator();

            ImGui::AlignFirstTextHeightToWidgets();
            ImGui::PushItemWidth(50);
            ImGui::PushAllowKeyboardFocus(false);
            int rows_backup = Rows;
            if (ImGui::DragInt("##rows", &Rows, 0.2f, 4, 32, "%.0f rows"))
            {
                ImVec2 new_window_size = ImGui::GetWindowSize();
                new_window_size.x += (Rows - rows_backup) * (cell_width + glyph_width);
                ImGui::SetWindowSize(new_window_size);
            }
            ImGui::PopAllowKeyboardFocus();
            ImGui::PopItemWidth();
            ImGui::SameLine();
            ImGui::Text("Range %0*X..%0*X", addr_digits_count, (int)base_display_addr, addr_digits_count, (int)base_display_addr+mem_size-1);
            ImGui::SameLine();
            ImGui::PushItemWidth(70);
            if (ImGui::InputText("##addr", AddrInput, 32, ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_EnterReturnsTrue))
            {
                int goto_addr;
                if (sscanf(AddrInput, "%X", &goto_addr) == 1)
                {
                    goto_addr -= base_display_addr;
                    if (goto_addr >= 0 && goto_addr < mem_size)
                    {
                        ImGui::BeginChild("##scrolling");
                        ImGui::SetScrollFromPosY(ImGui::GetCursorStartPos().y + (goto_addr / Rows) * ImGui::GetTextLineHeight());
                        ImGui::EndChild();
                        DataEditingAddr = goto_addr;
                        DataEditingTakeFocus = true;
                    }
                }
            }
            ImGui::PopItemWidth();
        }
        ImGui::End();
    }
};
*/
