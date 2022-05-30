use serenity::model::channel::Message;

/*
 * Agregar funcionalidad para que tambien compare el caracter discord con el que esta en el .env
 */
pub fn get_command(msg: &Message) -> String {
    // Remove first character of msg
    let mut input = String::from(&msg.content);
    input.remove(0);
    
    // Get first word of input
    let mut split_input = input.split_whitespace();

    // instruction is the first word in the message
    let instruction = split_input.next().unwrap();

    // return command
    String::from(instruction)
}