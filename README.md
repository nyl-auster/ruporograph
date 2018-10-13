# Serveur GraphQL avec Rust ( Travail en cours )

Je suis un noob en Rust 🤘 Ceci est une expérimentation 🔬 N'utilisez pas ça en production 😱

Ce serveur fait la glue entre les librairies suivantes :

- [postgres](https://github.com/sfackler/rust-postgres) pour la base de données
- [rocket](https://rocket.rs/) pour la gestion des requêtes http
- [Juniper](https://github.com/graphql-rust/juniper) pour créer le schéma GraphQL

## Getting started

- installer Rust en `1.31.0-nightly`
- installer postgres
- créer une base de donnée
- mettre à jour le fichier `src/config.toml` avec l'adresse de la base de données
