use crate::{
    client::Client,
    core::{
        query::{Comparator, Filter, QueryResponse},
        response::{EmailGetResponse, EmailSetResponse},
    },
};

use super::{import::EmailImportResponse, Email, Property};

impl Client {
    pub async fn email_import<T, U>(
        &mut self,
        raw_message: Vec<u8>,
        mailbox_ids: T,
        keywords: Option<T>,
        received_at: Option<i64>,
    ) -> crate::Result<Email>
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        let blob_id = self
            .upload(self.default_account_id(), None, raw_message)
            .await?
            .unwrap_blob_id();
        let mut request = self.build();
        let import_request = request
            .import_email()
            .email(blob_id)
            .mailbox_ids(mailbox_ids);

        if let Some(keywords) = keywords {
            import_request.keywords(keywords);
        }

        if let Some(received_at) = received_at {
            import_request.received_at(received_at);
        }

        let id = import_request.create_id();
        request
            .send_single::<EmailImportResponse>()
            .await?
            .created(&id)
    }

    pub async fn email_set_mailbox(
        &mut self,
        id: &str,
        mailbox_id: &str,
        set: bool,
    ) -> crate::Result<Option<Email>> {
        let mut request = self.build();
        request.set_email().update(id).mailbox_id(mailbox_id, set);
        request.send_single::<EmailSetResponse>().await?.updated(id)
    }

    pub async fn email_set_mailboxes<T, U>(
        &mut self,
        id: &str,
        mailbox_ids: T,
    ) -> crate::Result<Option<Email>>
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        let mut request = self.build();
        request.set_email().update(id).mailbox_ids(mailbox_ids);
        request.send_single::<EmailSetResponse>().await?.updated(id)
    }

    pub async fn email_set_keyword(
        &mut self,
        id: &str,
        keyword: &str,
        set: bool,
    ) -> crate::Result<Option<Email>> {
        let mut request = self.build();
        request.set_email().update(id).keyword(keyword, set);
        request.send_single::<EmailSetResponse>().await?.updated(id)
    }

    pub async fn email_set_keywords<T, U>(
        &mut self,
        id: &str,
        keywords: T,
    ) -> crate::Result<Option<Email>>
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        let mut request = self.build();
        request.set_email().update(id).keywords(keywords);
        request.send_single::<EmailSetResponse>().await?.updated(id)
    }

    pub async fn email_destroy(&mut self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_email().destroy([id]);
        request
            .send_single::<EmailSetResponse>()
            .await?
            .destroyed(id)
    }

    pub async fn email_get(
        &mut self,
        id: &str,
        properties: Option<Vec<Property>>,
    ) -> crate::Result<Option<Email>> {
        let mut request = self.build();
        let get_request = request.get_email().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<EmailGetResponse>()
            .await
            .map(|mut r| r.unwrap_list().pop())
    }

    pub async fn email_query(
        &mut self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<Vec<Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_email();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort.into_iter());
        }
        request.send_single::<QueryResponse>().await
    }
}
