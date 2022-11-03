use core::panic;
use std::process::{Command, Output};
use std::{fs, vec};

/**
 * El usuario debera tener instalado git y tener una cuenta en git como requisito
 * Debera crear una carpeta principal
 * y dentro de esta puede crear sub capertas por ejemeplo
 * /PS1 /PS2 /Gamecube
 * y dentro de estas subcarpetas puede tener mas carpetas y/o archivos
 * que quiera almacenar ya sean partidas guardadas o perfiles de gamepads
 * etc...
 */

fn main() {

    //comprueba si git esta instalado
    check_git();

    //comprueba si ya hay un usuario guardado
    let log = check_git_login();

    if log {
        git_upload();
    }else{
        let new_log = git_login();
        if new_log {
            git_upload()
        }else{
            panic!("Intente inciar sesion manualmente con git");
        }
    }


}

fn run_git(arguments: Vec<&str>) -> bool{
    let git_command = Command::new("git")
    .args(arguments).output();

    let result = match git_command{
        Ok(s) => true,
        _ => false
    };

    result
}

fn git_upload(){

    println!("Iniciando...");

    //agregar todos los archivos
    let add = run_git(vec!["add", "."]);
    match add{
        true => println!("Archivos agregados ✅"),
        _ => println!("Error archivos no agregados ❌") 
    }
   
    //mensaje archivos actualizados
    let commit = run_git(vec!["commit", "-m", "\"update\""]);
    match commit{
        true => println!("Procesando archivos ✅"),
        _ => println!("Error al procesar archivos ❌") 
    }
       
    //subir archivos actualizados
    let push = run_git(vec!["push"]);
    match push{
        true => println!("Archivos actualizados con exito ✅"),
        _ => println!("Error al subir los archvios ❌") 
    }
}

fn check_git_login() -> bool{
    
    //verifica si existe un usuario
    let git_user = run_git(vec!["config", "user.name"]);
    let git_email = run_git(vec!["config", "user.email"]);
    
    if git_user && git_email {
        return true
    }else{
        return false
    }
}

fn git_login() -> bool{

    let mut buf_user = String::new();
    let mut buf_email = String::new();
    let mut buf_password = String::new();

    println!("Ingresa tu usuario");
    user_input(&mut buf_user);

    println!("Ingresa tu email");
    user_input(&mut buf_email);
    
    println!("Ingresa un token");
    user_input(&mut buf_password);

    if !check_empty(&buf_user) 
    && !check_empty(&buf_email) 
    && !check_empty(&buf_password)
    {
        let user = run_git(vec!["config", "--global", "user.name", &buf_user]);
        let email = run_git(vec!["config", "--global", "user.email", &buf_email]);
        let password =run_git(vec!["config", "--global", "user.password", &buf_password]);
        let store =run_git(vec!["config", "--global", "credential.helper", "store"]);
    

        if user && email && password && store{
            println!("Inicio de sesión exitoso ✅");
            true
        }else{
            println!("Revise su información");
            false
        }
    }else{
        false
    }
}

fn user_input(buf: &mut String) -> bool{
    match std::io::stdin()
    .read_line(buf){
        Ok(n) => true,
        _=> false,
    }
}

fn check_git(){
    println!("Comprobando instalacion de GIT....");
    let git = run_git(vec![""]);
    if git {
        println!("Comprobado ✅")
    }else{
        panic!("❌ Instala GIT para utilizar este programa")
    }
}


fn check_empty(argument: &String) -> bool{
    if !argument.is_empty(){
        false
    }else{
        true
    }
}