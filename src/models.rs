use serde::Deserialize;

// Definerer strukturen 'Route' som repræsenterer en rute i applikationen
#[derive(Deserialize)]
pub struct Route {
    // Stien for ruten (f.eks. "/add")
    pub path: String,
    // Navnet på den funktion, der håndterer ruten
    pub function: String,
}