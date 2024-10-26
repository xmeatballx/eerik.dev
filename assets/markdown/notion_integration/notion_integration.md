# Making a custom integration for your Notion workspace

## What is the Notion API?

Notion is an extremely powerful knowledge management system and productivity tool. However you may find cases where you have a goal that is over-complicated or impossible to accomplish in notion. 

If only there was a way to get access to all of your Notion data from an external environment where you can do whatever you want with it… There is! It’s called the Notion API (in this tutorial we will be looking specifically at the Javascript SDK). 

With the Notion API you can create, read, or update any page, database, block, or user in your Notion workspace using code. The API provides a bounty of straightforward, readable functions that interact with you workspace. For example, to retrieve a page from your workspace the Javascript code looks like this:

```jsx
const page = await notion.pages.retrieve({ page_id: pageId })
```

This code will return a page metadata object that includes the page title, cover image, properties, associated users, and more. What you do with this data is completely up to you, but in this tutorial we will be using the API to get a random quote from a Notion database of book excerpts. We will display this quote on a webpage that can be embedded as a widget in Notion.

## Setup

⚠️ This tutorial requires that you have recent versions of NodeJS and NPM installed on your machine

### Create a Notion API key

An API key is a unique alphanumeric token that is used to authenticate your server’s requests to Notion. To create the API key for your integration go to the Settings and Members tab from your Notion sidebar and select Integrations. At the bottom of the page click the link that says “Develop your own integration”

![diy.png](/assets/markdown/notion_integration/diy.png)

Enter the project info and submit. You will be redirected to a page with your API key. Write this down somewhere safe.

## Creating the Project

In a terminal create a directory to store your project

```jsx
mkdir my-notion-integration
```

Navigate to the project directory

```jsx
cd my-notion-integration
```

Create the following file structure

- public
    - index.html
- app.js
- .env
- .gitignore
- README.md

Initialize npm

```jsx
npm init -y
```

Install express for server, dotenv for securing API key, and the notion api SDK

```jsx
npm i express dotenv notionhq/client -s
```

Add your notion API key to your .env file

```jsx
NOTION_API_KEY={your_api_key}
```

Add node modules and .env to .gitignore

## Creating your Notion Client

This is how we create the Notion client. The client instance takes your API key that we stored in the .env file before.

```jsx
import { Client } from '@notionhq/client';
const notion = new Client({ auth: process.env.NOTION_ACCESS_TOKEN });
```

Once our client is initialized we can start querying our workspace

```jsx
const response = await notion.search({ query: 'excerpts' });
const db_id = response.results[0].id;
const excerpts = await notion.databases.query({ database_id: db_id });
```

We start by searching our workspace for the database we want. This search will return an array of items that can either be a page, database, block, or user. In this case I know that the only thing in my workspace named “Excerpts” is a single database, so I can grab the first search result (response.results[0]) and extract the database id from that. Once you have the database id you can query all items in that database using the notion.databases.query function. 

## Server Setup

In the app.js file add the following code

```jsx
import dotenv from 'dotenv';
dotenv.config();

import express from 'express';
const app = express();
const port = 3000;
app.listen(port);
```

This sets up dotenv so that you can bring your API key in from a private file and initializes express, which is used to configure routes for your server

 

Add this snippet below the code in your app.js file

```jsx
app.use(express.static('public'))
```

This tells express to serve static files from a directory called “public”. The public folder will house the HTML document that renders our embeddable web page

## The data route

Here we are initializing a new route called data. This will return a json object containing the excerpt, author, chapter, and page number.

```jsx
app.get('/data', async (req, res) => {
  const randomIndex = Math.round(Math.random() * excerpts.results.length);
  const page_id = excerpts.results[randomIndex].id;
  const page = await notion.pages.retrieve({ page_id: page_id });
  const content = page.properties.Excerpt.rich_text[0].plain_text;
  const book = await notion.pages.retrieve({ page_id: page.properties.Book.relation[0].id });
  const bookTitle = book.properties.Name.title[0].plain_text;
  const chapter = await notion.pages.retrieve({ page_id: page.properties.Chapter.relation[0].id });
  const chapterName = chapter.properties.Name.title[0].plain_text;
  const pageNumber = page.properties.Page.number;
  const data = {
    book: bookTitle,
    chapter: chapterName,
    page: pageNumber,
    content: content,
  };
  return res.json(data);
});
```

This may look like a lot of code but there’s only a couple things happening here. First we are getting a random number between 0 and the total number of items in your database. With that number we can select a random page within the database and retrieve its id. With this id we can get all of the page properties aka the info that we would like to display on our embeddable web page

## Creating the embeddable web page

## HTML

In your index.html file add the following code

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
</head>
<body>
    <h2 class="loader">Loading</h2>
    <figure>
      <blockquote class="quote suspense"></blockquote>
      <figcaption class="source suspense"></figcaption>
    </figure>
</body>
</html>
```

Here we are setting up the elements in which we will display our content from notion. Since this is a book excerpt I used figure element that will render a blockquote and a caption. The “Loading” header will display temporarily while our data is being fetched from our server. 

## Style

Most of our styling will be handled by [Open Props Normalize](https://codepen.io/argyleink/pen/KKvRORE). Open props is a library of premade CSS variables that help you write CSS and maintain consistency across your design. Open props normalize provides a well organized starting point for styling. It automatically applies type and color systems to your page which match the look of Notion quite well. With Open Props Normalize as a base we barely have to write any CSS. Place this code in the HTML document head

```html
<style>
      @import 'https://unpkg.com/open-props';
      @import 'https://unpkg.com/open-props/normalize.min.css';
      @import 'https://unpkg.com/open-props/buttons.min.css';

      html {
        --surface-1: #191919;
      }

      .loader {
        text-align: center;
        max-inline-size: 100vw;
      }

      .suspense {
        border: none;
        transform: scale(0);
        opacity: 0;
      }

      * {
        transition: all 0.5s;
      }
</style>
```

Here we are loading Open Props and changing the default dark mode background color to match Notion and adding some flare to the loading process with a CSS transition.

## Javascript

Our front end Javascript code uses the built in fetch API to grab the JSON object generated on our server and inject the content into our predefined HTML structure. Place this before the closing body tag.

```jsx
const loader = document.querySelector('.loader');
const quote = document.querySelector('.quote');
const source = document.querySelector('.source');

fetch('/data')
  .then((res) => res.json())
  .then((data) => {
    loader.style.display = 'none';
    quote.textContent = data.content;
    quote.classList.remove('suspense')
    source.textContent = `${data.book}, ${data.chapter}, p. ${data.page}`;
    source.classList.remove('suspense');
  });
```

We get references to our HTML elements and make a fetch request to our server. Then we parse our JSON object and inject the data into the DOM.

## Running our app

To see your integration in action go to a terminal, navigate to your project directory and run 

```jsx
node app.js
```

Then go to [localhost:3000](http://localhost:3000) in your browser to see the web page.

# Conclusion

Now we have a web page that displays a random database entry from Notion, however you may notice that you are unable to embed a localhost page. In the next tutorial we will cover hosting your app for free on Heroku. This will provide you with a URL which is compatible with the Notion embed functionality.
