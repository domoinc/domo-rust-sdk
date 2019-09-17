use serde::{Deserialize, Serialize};
use std::error::Error;

/// The page object is a screen where you can view a “collection” of data, which is typically displayed in cards.
/// You use a page to organize, manage, and share content to other users in Domo.
/// Pages allow you to send external reports, create holistic filters across all metrics within the page, or have conversations in Domo’s Buzz tool about the data associated to the entire page.
/// The Page API allows you to create, delete,  retrieve a page or a list of pages, and update page information and content within a page.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Page {
    /// The id of the page
    pub id: Option<u64>,

    /// The name of the page
    pub name: Option<String>,

    /// The ID of the page that is higher in organizational hierarchy
    pub parent_id: Option<u64>,

    /// The ID of the page owner
    pub owner_id: Option<u64>,

    /// Determines whether users (besides the page owner) can make updates to page or its content - the default value is false
    pub locked: Option<bool>,

    /// The IDs of collections within a page
    pub collection_ids: Option<Vec<u64>>,

    /// The ID of all cards contained within the page
    pub card_ids: Option<Vec<u64>>,

    /// All pages that are considered "sub pages" in organizational hierarchy
    pub children: Option<Vec<Page>>,

    /// Determines the access given to both individual users or groups within Domo
    pub visibility: Option<Visibility>,
}

/// Shares pages with users and/or groups
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Visibility {
    /// IDs provided will share page with associated users
    pub user_ids: Option<Vec<u64>>,

    /// IDs provided will share page with associated groups
    pub group_ids: Option<Vec<u64>>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            parent_id: None,
            owner_id: None,
            locked: None,
            collection_ids: None,
            card_ids: None,
            children: None,
            visibility: None,
        }
    }

    pub fn template() -> Self {
        Self {
            id: Some(0),
            name: Some(String::from("Page Name")),
            parent_id: Some(0),
            owner_id: Some(0),
            locked: Some(false),
            collection_ids: Some(vec![1, 2, 3]),
            card_ids: Some(vec![1, 2, 3]),
            children: Some(vec![]),
            visibility: Some(Visibility {
                user_ids: Some(vec![1, 2, 3]),
                group_ids: Some(vec![1, 2, 3]),
            }),
        }
    }
}

/// Represents a smaller subset of cards with a header on a page
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Collection {
    /// The id of the collection
    pub id: Option<u64>,

    /// Page collection's name displayed above the set of cards
    pub title: Option<String>,

    /// Additional text within the page collection
    pub description: Option<String>,

    /// IDs provided will add or remove cards that are not a part of a page collection
    pub card_ids: Option<Vec<u64>>,
}

impl Collection {
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            description: None,
            card_ids: None,
        }
    }

    pub fn template() -> Self {
        Self {
            id: Some(0),
            title: Some(String::from("Collection Title")),
            description: Some(String::from("Collection Description")),
            card_ids: Some(vec![1, 2, 3]),
        }
    }
}

/// Page API methods
/// Uses the form method_object
impl super::Client {
    /// Get a list of all pages in your Domo instance.
    pub fn get_pages(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Page>, Box<dyn Error>> {
        let at = self.get_access_token("dashboard")?;
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = offset {
            q.push(("offset", v.to_string()));
        }
        Ok(self
            .client
            .get(&format!("{}{}", self.host, "/v1/pages"))
            .query(&q)
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Creates a new page in your Domo instance.
    pub fn post_page(&self, page: Page) -> Result<Page, Box<dyn Error>> {
        let at = self.get_access_token("dashboard")?;
        Ok(self
            .client
            .post(&format!("{}{}", self.host, "/v1/pages"))
            .header("Authorization", at)
            .json(&page)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Retrieves the details of an existing page.
    pub fn get_page(&self, id: u64) -> Result<Page, Box<dyn Error>> {
        let at = self.get_access_token("dashboard")?;
        Ok(self
            .client
            .get(&format!("{}{}{}", self.host, "/v1/pages/", id))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Updates the specified page by providing values to parameters passed.
    /// Any parameter left out of the request will cause the specific page’s attribute to remain unchanged.
    ///
    /// Also, collections cannot be added or removed via this endpoint, only reordered.
    /// Giving access to a user or group will also cause that user or group to have access to the parent page (if the page is a subpage).
    /// Moving a page by updating the parentId will also cause everyone with access to the page to have access to the new parent page.
    pub fn put_page(&self, id: u64, page: Page) -> Result<Page, Box<dyn Error>> {
        let at = self.get_access_token("dashboard")?;
        Ok(self
            .client
            .put(&format!("{}{}{}", self.host, "/v1/pages/", id))
            .header("Authorization", at)
            .json(&page)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Permanently deletes a page from your Domo instance.
    /// This is destructive and cannot be reversed.
    pub fn delete_page(&self, id: u64) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("dashboard")?;
        self.client
            .delete(&format!("{}{}{}", self.host, "/v1/pages/", id))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn get_page_collections(&self, id: u64) -> Result<Vec<Collection>, Box<dyn Error>> {
        let at = self.get_access_token("dashboard")?;
        Ok(self
            .client
            .get(&format!(
                "{}{}{}{}",
                self.host, "/v1/pages/", id, "/collections"
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn post_page_collection(
        &self,
        id: u64,
        collection: Collection,
    ) -> Result<Collection, Box<dyn Error>> {
        let at = self.get_access_token("dashboard")?;
        Ok(self
            .client
            .post(&format!(
                "{}{}{}{}",
                self.host, "/v1/pages/", id, "/collections"
            ))
            .header("Authorization", at)
            .json(&collection)
            .send()?
            .error_for_status()?
            .json()?)
    }

    pub fn put_page_collection(
        &self,
        id: u64,
        collection_id: u64,
        collection: Collection,
    ) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("dashboard")?;
        self.client
            .put(&format!(
                "{}{}{}{}{}",
                self.host, "/v1/pages/", id, "/collections/", collection_id
            ))
            .header("Authorization", at)
            .json(&collection)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn delete_page_collection(
        &self,
        id: u64,
        collection_id: u64,
    ) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("dashboard")?;
        self.client
            .delete(&format!(
                "{}{}{}{}{}",
                self.host, "/v1/pages/", id, "/collections/", collection_id
            ))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}
