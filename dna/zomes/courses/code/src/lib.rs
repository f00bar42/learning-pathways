/***************** Required Library */
#![feature(vec_remove_item)]
#![allow(dead_code)]
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::prelude::*;

use hdk::holochain_persistence_api::cas::content::Address;
use hdk::AGENT_ADDRESS;
use hdk_proc_macros::zome;

//use std::convert::TryInto;

/******************************** */
mod content;
mod course;
mod section;
mod anchor_trait;

#[zome]
mod course_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    fn get_my_address() -> ZomeApiResult<Address> {
        Ok(AGENT_ADDRESS.clone())
    }

    /**************************** Course Entry Definition and Functions */
    #[entry_def]
    fn anchor_entry_definition() -> ValidatingEntryType {
        course::anchor::anchor_entry_def()
    }

    #[entry_def]
    fn course_entry_definition() -> ValidatingEntryType {
        course::entry::course_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_course(title: String, timestamp: u64) -> ZomeApiResult<Address> {
        course::handlers::create(title, timestamp)
    }

    #[zome_fn("hc_public")]
    fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    #[zome_fn("hc_public")]
    fn update_course(
        title: String,
        sections_addresses: Vec<Address>,
        course_address: Address,
    ) -> ZomeApiResult<Address> {
        course::handlers::update(title, sections_addresses, &course_address)
    }

    #[zome_fn("hc_public")]
    fn delete_course(course_address: Address) -> ZomeApiResult<Address> {
        course::handlers::delete(course_address)
    }

    #[zome_fn("hc_public")]
    fn get_all_courses() -> ZomeApiResult<Vec<Address>> {
        course::handlers::list()
    }
    #[zome_fn("hc_public")]
    fn get_my_courses() -> ZomeApiResult<Vec<Address>> {
        course::handlers::get_my_courses()
    }

    #[zome_fn("hc_public")]
    fn get_my_enrolled_courses() -> ZomeApiResult<Vec<Address>> {
        course::handlers::get_my_enrolled_courses()
    }

    #[zome_fn("hc_public")]
    fn enrol_in_course(course_address: Address) -> ZomeApiResult<Address> {
        course::handlers::enrol_in_course(course_address)
    }

    #[zome_fn("hc_public")]
    fn get_all_students(course_address: Address) -> ZomeApiResult<Vec<Address>> {
        course::handlers::get_students(course_address)
    }

    /**************************** Section Entry Definition & Functions */
    #[entry_def]
    fn section_anchor_entry_definition() -> ValidatingEntryType {
        section::anchor::section_anchor_def()
    }

    #[entry_def]
    fn section_entry_definition() -> ValidatingEntryType {
        section::entry::entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_section(
        title: String,
        course_address: Address,
        timestamp: u64,
    ) -> ZomeApiResult<Address> {
        section::handlers::create(title, &course_address, timestamp)
    }

    #[zome_fn("hc_public")]
    fn update_section(title: String, section_anchor_address: Address) -> ZomeApiResult<Address> {
        section::handlers::update(title, &section_anchor_address)
    }

    #[zome_fn("hc_public")]
    fn delete_section(section_anchor_address: Address) -> ZomeApiResult<Address> {
        section::handlers::delete(section_anchor_address)
    }

    /**************************** Content Zome Functions */
    #[entry_def]
    fn content_entry_definition() -> ValidatingEntryType {
        content::entry::section_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_content(
        name: String,
        section_anchor_address: Address,
        url: String,
        timestamp: u64,
        description: String,
    ) -> ZomeApiResult<Address> {
        content::handlers::create(name, section_anchor_address, url, timestamp, description)
    }

    #[zome_fn("hc_public")]
    fn get_contents(section_anchor_address: Address) -> ZomeApiResult<Vec<Address>> {
        content::handlers::get_contents(&section_anchor_address)
    }
    #[zome_fn("hc_public")]
    fn delete_content(content_address: Address) -> ZomeApiResult<Address> {
        content::handlers::delete(content_address)
    }

    #[zome_fn("hc_public")]
    fn update_content(
        content_address: Address,
        name: String,
        url: String,
        description: String,
    ) -> ZomeApiResult<Address> {
        content::handlers::update(content_address, name, url, description)
    }
}
