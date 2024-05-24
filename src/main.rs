use mongodb::{bson::doc, options::{ClientOptions, ResolverConfig,FindOptions}, Client};
use std::env;
use dotenv::dotenv;
use std::error::Error;
use bson::oid::ObjectId;
use tokio::{self, stream::StreamExt};
#[tokio::main]

async fn main()-> Result<(),Box<dyn Error>> {
    // To get the access of all the env variables
    dotenv().ok();
    let uri: String = env::var("MONGODB_URL")?;
    let options = ClientOptions::parse_with_resolver_config(&uri, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;

    // To print the name of all databases
    for name in client.list_database_names(None, None).await?{
        println!("-{}", name);
    }

    // It will create an instance of the collection that we are trying to make 
    let receipes = client.database("sample_mflix").collection("books");
    
    // Insert into collection
    receipes.insert_one(doc!{
        "name":"The second name of the book",
        "author": "The second name of the book",
        "issue_date":"20-March-2022"
    },None).await?;

    // To print the details of the data in the collection
    let mut cursor = receipes.find(None, None).await?;
    while let Some(result) = cursor.next().await{
        let receipe = result?;
        println!("Name: {}", receipe.get_str("name")?);
    }

    // To find the details according to a particular data 
    let find_options = FindOptions::default();
    let query = doc!{
        "name":"The second name of the book"
    };
    let mut cursor = receipes.find(query, find_options).await?;
    while let Some(result) = cursor.next().await {
        let book = result?;
        println!("Found Name: {}", book.get_str("author")?);
    }

    // To update the details 
    let _id = ObjectId::with_string("665097972a73ed26fc060c43")?;
    receipes.update_one(doc!{"_id":_id},doc!{"$set":{"author":"The second name of the second author"}}, None).await?;
    Ok(())
}