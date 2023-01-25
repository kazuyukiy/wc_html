// use crate::page5;

// use crate::page5_system_update_0_0_1_to_0_0_3;
// use crate::page5_system_update_old_to_0_0_3;
// use crate::page5_system_update_0_0_2_to_0_0_3;

// super::page;

// pub fn update(page: &mut page5::Page) {
pub fn update(page: &mut super::Page) {
    // println!("page5_system_update.rs page5_system_update_0_0_2_to_0_0_3 is not devaloped completely yet");

    // wc pages are in utf8 .
    // If it is not utf8, you can skip the update.
    // utf8 check might be done also in each sub procedures,
    // eventhough, it is worth to check here to skip the rest in earlier .

    if let None = page.file().content_str() {
        return;
    }

    let mut updated = false;

    // updated = page5_system_update_old_to_0_0_3::update(page) || updated;
    updated = super::page_system_update_old_to_0_0_3::update(page) || updated;
    // updated = page5_system_update_0_0_1_to_0_0_3::update(page) || updated;
    updated = super::page_system_update_0_0_1_to_0_0_3::update(page) || updated;

    // Considering discon page5_system_update_0_0_2_to_0_0_3
    // It is only adding data.href that should not need
    // because the program can be coded to handle without data.href .
    // updated = page5_system_update_0_0_2_to_0_0_3::update(page) || updated;

    if updated {
        // DBG
        // println!("page5`_system_update.rs updated is ture ");

        page.page_current_save();

        page.page_json_update_save();
    }
} // end of fn update
