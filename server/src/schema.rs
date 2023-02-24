use async_graphql::{ComplexObject, Context, Object};
use sea_orm::*;

use crate::entities::{prelude::*, *};

pub(crate) struct QueryRoot;
pub(crate) struct MutationRoot;

#[Object]
impl QueryRoot {
    async fn albums(&self, ctx: &Context<'_>) -> Result<Vec<albums::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Albums::find().all(db).await
    }

    async fn artists(&self, ctx: &Context<'_>) -> Result<Vec<artists::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Artists::find().all(db).await
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<users::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Users::find().all(db).await
    }

    async fn songs(&self, ctx: &Context<'_>) -> Result<Vec<songs::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Songs::find().all(db).await
    }

    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<tags::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Tags::find().all(db).await
    }
}

#[Object]
impl MutationRoot {
    async fn add_album(
        &self, 
        ctx:       &Context<'_>, 
        title:     String, 
        image_url: Option<String>,
        details:   Option<String>,
        artist_id: i32,
    ) -> Result<albums::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Albums::insert(albums::ActiveModel {
            title:     ActiveValue::Set(title),
            image_url: ActiveValue::Set(image_url),
            details:   ActiveValue::Set(details),
            artist_id: ActiveValue::Set(artist_id),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Albums::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|a| a.unwrap())
    }

    async fn add_artist(
        &self, 
        ctx:       &Context<'_>, 
        email:     String, 
        username:  String,
        image_url: Option<String>,
        bio:       Option<String>,
        password:  String,
        tags:      Option<Vec<String>>,
    ) -> Result<artists::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Artists::insert(artists::ActiveModel {
            email:     ActiveValue::Set(email),
            username:  ActiveValue::Set(username),
            image_url: ActiveValue::Set(image_url),
            bio:       ActiveValue::Set(bio),
            password:  ActiveValue::Set(password),
            ..Default::default()
        })
        .exec(db)
        .await?;

        let artist = Artists::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|a| a.unwrap());

        // let tags = tags.map(|t| t.to_lowercase())

        // for tag in tags.into_iter() {
        //     let t = Tags::find()
        //         .filter(tags::column::Name.contains(&tag))
        //         .one(db)
        //         .await?;
            
        //     println!(t);
        // }
    

        artist
    }

    async fn add_user(
        &self, 
        ctx:       &Context<'_>, 
        email:     String, 
        username:  String,
        image_url: Option<String>,
        password:  String,
    ) -> Result<users::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Users::insert(users::ActiveModel {
            email:     ActiveValue::Set(email),
            username:  ActiveValue::Set(username),
            image_url: ActiveValue::Set(image_url),
            password:  ActiveValue::Set(password),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Users::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|u| u.unwrap())
    }

    async fn add_song(
        &self, 
        ctx:          &Context<'_>, 
        title:        String,
        file_url:     String,
        track_length: String,
        image_url:    Option<String>,
        artist_id:    i32,
        album_id:     Option<i32>
    ) -> Result<songs::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Songs::insert(songs::ActiveModel {
            title:        ActiveValue::Set(title),
            file_url:     ActiveValue::Set(file_url),
            track_length: ActiveValue::Set(track_length),
            image_url:    ActiveValue::Set(image_url),
            artist_id:    ActiveValue::Set(artist_id),
            album_id:     ActiveValue::Set(album_id),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Songs::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|s| s.unwrap())
    }
}

#[ComplexObject]
impl artists::Model {
    async fn albums(&self, ctx: &Context<'_>) -> Result<Vec<albums::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Albums).all(db).await
    }

    async fn songs(&self, ctx: &Context<'_>) -> Result<Vec<songs::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Songs).all(db).await
    }

    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<tags::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Tags).all(db).await
    }
}

#[ComplexObject]
impl albums::Model {
    async fn songs(&self, ctx: &Context<'_>) -> Result<Vec<songs::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Songs).all(db).await
    }
}

#[ComplexObject]
impl users::Model {
    async fn albums(&self, ctx: &Context<'_>) -> Result<Vec<albums::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Albums).all(db).await
    }

    async fn songs(&self, ctx: &Context<'_>) -> Result<Vec<songs::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Songs).all(db).await
    }
}

#[ComplexObject]
impl tags::Model {
    async fn albums(&self, ctx: &Context<'_>) -> Result<Vec<albums::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Albums).all(db).await
    }

    async fn artists(&self, ctx: &Context<'_>) -> Result<Vec<artists::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(Artists).all(db).await
    }
}
