// .setup(|app| {
        //     let file_menu = SubmenuBuilder::new(app, "File")
        //         .text("open", "Open")
        //         .text("quit", "Quit")
        //         .build()?;

        //     let lang_str = "en";
        //     let check_sub_item_1 = CheckMenuItemBuilder::new("English")
        //         .id("en")
        //         .checked(lang_str == "en")
        //         .build(app)?;

        //     let check_sub_item_2 = CheckMenuItemBuilder::new("Chinese")
        //         .id("en")
        //         .checked(lang_str == "en")
        //         .enabled(false)
        //         .build(app)?;

        //     let icon_image = Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap();

        //     let icon_item = IconMenuItemBuilder::new("icon")
        //         .icon(icon_image)
        //         .build(app)?;

        //     let other_item = SubmenuBuilder::new(app, "language")
        //         .item(&check_sub_item_1)
        //         .item(&check_sub_item_2)
        //         .build()?;

        //     let menu = MenuBuilder::new(app)
        //         .items(&[&file_menu, &icon_item, &other_item])
        //         .build()?;

        //     app.set_menu(menu)?;

        //     Ok(())
        // })