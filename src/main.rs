use std::{process::{Command, exit, Output}, str::from_utf8};

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
    let log = is_login();
    println!("Existe: {:?}", log);

    if log {
        git_upload();
    }else{
        let new_log = git_login();
        if new_log {
            git_upload();
        }else{
            exit(0);
        }
    }

}

fn run_git(arguments: Vec<&str>) -> Output{
    let git_command = Command::new("git")
    .args(arguments).output();

    let result = match git_command{
        Ok(res) => res,
        _=> panic!("❌ Reportar el error❌")
    };

    result
}

fn git_upload(){

    println!("🔥 Iniciando la aplicación 🔥");
    //recuperar archivos actualizados
    /* let pull = run_git(vec!["pull"]);
    match pull{
        true => println!("- Descargando archivos 🎲"),
        _ => println!("Error al descargar los archivos ❌") 
    }
    */

    //agregar todos los archivos
    let add = run_git(vec!["add", "."]);
    println!("- Archivos agregados 🥪");

   
    //mensaje archivos actualizados
    let commit = run_git(vec!["commit", "-m updated"]);
    println!("- Procesando archivos ⏰");
       
    //subir archivos actualizados
    let push = run_git(vec!["push"]);
    println!("- Archivos actualizados con exito 🥂");

}

fn is_login() -> bool{
    
    //verifica si existe un usuario
    let git_user = run_git(vec!["config", "--global", "user.name"]);
    let git_email = run_git(vec!["config", "--global", "user.email"]);
    
    let res_user = handle_output(&git_user);
    let res_email = handle_output(&git_email);
    if check_empty(&res_user) && check_empty(&res_email){
        false
    }else{
        true
    }
}

fn handle_output(values: &Output) -> String{
    let new_value = from_utf8(&values.stdout).unwrap().to_string();
    new_value.trim().to_string()
}


fn command_status(){

}

fn git_login() -> bool{
    let clean = run_git(vec!["config", "--global", "--unset", "credential.helper"]);

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
    
        let user = handle_output(&user);
        let email = handle_output(&email);
        let password = handle_output(&password);
        println!("{:?}", store);
        if !check_empty(&user.to_string())
        && !check_empty(&email.to_string())
        && !check_empty(&password.to_string())
        {
            println!("Inicio de sesión exitoso 💡");
            true
        }else{
            println!("Revise su información");
            false
        }

    }else{
        println!("❌ Error ❌");
        println!("- Credenciales vacias vuelva a ejecutar el app ❌");
        println!("- Intente inciar sesión manualmente con git ❌");
        false
    }
}

fn user_input(buf: &mut String){
    match std::io::stdin()
    .read_line(buf){
        Ok(n) => true,
        _=> false,
    };
}

fn check_git(){
    println!("🚀 Comprobando instalación de GIT 🚀");
    let git = run_git(vec!["--version"]);
    let res = from_utf8(&git.stdout).unwrap();
    if res.len() > 0{
        println!("- Comprobado 🔥")
    }else{
        println!("❌ Instala GIT para utilizar este programa");
        exit(0)
    }
}


fn check_empty(argument: &String) -> bool{
    if argument.is_empty(){
        true
    }else{
        false
    }
}