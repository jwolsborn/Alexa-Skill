extern crate lambda_runtime as lambda;
extern crate alexa_sdk;

use lambda::{lambda, Context, error::HandlerError};
use alexa_sdk::{Request,Response};
use alexa_sdk::request::{IntentType};
use std::error::Error;

//Handles when asking blazer skill for help
fn handle_help(_req: &Request) -> Result<Response,HandlerError>{ 

    Ok(Response::new_simple("hello", "Ask me a question about the Blazers"))
}

//default when no matching intent type is found it ends the skill
fn handle_cancel(_req: &Request) -> Result<Response,HandlerError> {

    Ok(Response::end())

}

//handles when asking how many games a player has played this season
fn handle_games(req: &Request) -> Result<Response,HandlerError> {

	if let Some(ref s) = req.slot_value("name") {
		match s.to_lowercase().as_str(){
			"lillard"|"damian"|"damian Lillard" => Ok(Response::new_simple("games","Damian Lillard played in 80 games in the regular season")),
			"c.j."|"mccullum"|"c.j. mccullum" => Ok(Response::new_simple("games","CJ McCollum played in 70 games in the regular season")),
			"al farouq"|"aminu"|"al farouq aminu" => Ok(Response::new_simple("games","Al-Farouq Aminu played in 81 games in the regular season")),
			"evan" | "turner" | "evan turner" => Ok(Response::new_simple("games","Evan Turner played in 73 games in the regular season")),
			"jake" | "layman" | "jake layman" => Ok(Response::new_simple("games","Jake Layman played in 71 games in the regular season")),
			"myers"|"leonard"|"myers Leonard" => Ok(Response::new_simple("games","Meyers Leonard played in 61 games in the regular season")),
			"scott" | "lagusse" | "scott lagusse" => Ok(Response::new_simple("games","Skal Labissiere played in 22 games in the regular season")),
			"maurice" | "mo" |"harkless"| "maurice harkless" | "mo harkless" => Ok(Response::new_simple("games","Mo Harkless played in 60 games in the regular season")),
			"zach" | "collins" | "zach collins" => Ok(Response::new_simple("games","Zach Collins played in 77 games in the regular season")),
			"anthony" | "simmons" | "anthony simmons" => Ok(Response::new_simple("games","Anfernee Simons played in 20 games in the regular season")),
			"yusuf" | "yusuf ergin" | "ner berkovic" => Ok(Response::new_simple("games","Jusuf Nurkic played in 72 games in the regular season")),
			"gary" | "gary trent junior" | "gary trent" => Ok(Response::new_simple("games","Gary Trent Junior played in 15 games in the regular season")),
			"seth" | "curry" | "seth curry" => Ok(Response::new_simple("games","Seth Curry played in 74 games in the regular season")),
			"rodney" | "hood" | "rodney hood" => Ok(Response::new_simple("games","Rodney Hood played in 72 games in the regular season")),
			"enes" | "kanter" | "enes kanter" => Ok(Response::new_simple("games","Enes Kanter played in 67 games in the regular season")),
			"the team" => Ok(Response::new_simple("games","The Blazers, as a team, played all 82 games of the regular season")),	
			_ => Ok(Response::new_simple("games","I don't recognize that players name, please try again")),
		}
	}else{
		Ok(Response::new_simple("games","invalid"))
	}
			
}

//Handles when asked how many rebounds a player has had
fn handle_rbs(req: &Request) -> Result<Response,HandlerError> {

	if let Some(ref s) = req.slot_value("name") {
		match s.to_lowercase().as_str(){
			"lillard"|"damian"|"damian Lillard" => Ok(Response::new_simple("rebounds","Damian Lillard averaged 4.6 rebounds in the regular season")),
			"c.j."|"mccullum"|"c.j. mccullum" => Ok(Response::new_simple("rebounds","CJ McCollum averaged 4 rebounds in the regular season")),
			"al farouq"|"aminu"|"al farouq aminu" => Ok(Response::new_simple("rebounds","Al-Farouq Aminu averaged 7.4 rebounds in the regular season")),
			"evan" | "turner" | "evan turner" => Ok(Response::new_simple("rebounds","Evan Turner averaged 4.5 rebounds in the regular season")),
			"jake" | "layman" | "jake layman" => Ok(Response::new_simple("rebounds","Jake Layman averaged 3.1 rebounds in the regular season")),
			"myers"|"leonard"|"myers Leonard" => Ok(Response::new_simple("rebounds","Meyers Leonard averaged 3.8 rebounds per game in the regular season")),
			"scott" | "lagusse" | "scott lagusse" => Ok(Response::new_simple("rebounds","Skal Labissiere averaged 2 rebounds in the regular season")),
			"maurice" | "mo" |"harkless"| "maurice harkless" | "mo harkless" => Ok(Response::new_simple("rebounds","Mo Harkless averaged 4.5 rebounds in the regular season")),
			"zach" | "collins" | "zach collins" => Ok(Response::new_simple("rebounds","Zach Collins averaged 4.2 rebounds in the regular season")),
			"anthony" | "simmons" | "anthony simmons" => Ok(Response::new_simple("rebounds","Anfernee Simons averaged 0.7 rebounds in the regular season")),
			"yusuf" | "yusuf ergin" | "ner berkovic" => Ok(Response::new_simple("rebounds","Jusuf Nurkic averaged 10.4 rebounds in the regular season")),
			"gary" | "gary trent junior" | "gary trent" => Ok(Response::new_simple("rebounds","Gary Trent Junior averaged 0.7 rebounds in the regular season")),
			"seth" | "curry" | "seth curry" => Ok(Response::new_simple("rebounds","Seth Curry averaged 1.6 rebounds in the regular season")),
			"rodney" | "hood" | "rodney hood" => Ok(Response::new_simple("rebounds","Rodney Hood averaged 2.2 rebounds in the regular season")),
			"enes" | "kanter" | "enes kanter" => Ok(Response::new_simple("rebounds","Enes Kanter averaged 9.8 rebounds in the regular season")),
			"the team" => Ok(Response::new_simple("rebounds","The Blazers, as a team, averaged 10.2 points per player in the regular season")),	
			_ => Ok(Response::new_simple("rebounds","I don't recognize that players name, please try again")),
		}
	}else{
		Ok(Response::new_simple("rebounds","invalid"))
	}
			
}

//Handles when asked how many points a player scored
fn handle_points(req: &Request) -> Result<Response,HandlerError> {

	if let Some(ref s) = req.slot_value("name") {
		match s.to_lowercase().as_str(){
			"lillard"|"damian"|"damian Lillard" => Ok(Response::new_simple("points","Damian Lillard averaged 25.8 points in the regular season")),
			"c.j."|"mccullum"|"c.j. mccullum" => Ok(Response::new_simple("points","CJ McCollum averaged 21 points in the regular season")),
			"al farouq"|"aminu"|"al farouq aminu" => Ok(Response::new_simple("points","Al-Farouq Aminu averaged 9.4 points in the regular season")),
			"evan" | "turner" | "evan turner" => Ok(Response::new_simple("points","Evan Turner averaged 6.8 points in the regular season")),
			"jake" | "layman" | "jake layman" => Ok(Response::new_simple("points","Jake Layman averaged 7.6 points in the regular season")),
			"myers"|"leonard"|"myers Leonard" => Ok(Response::new_simple("points","Meyers Leonard averaged 5.9 points per game in the regular season")),
			"scott" | "lagusse" | "scott lagusse" => Ok(Response::new_simple("points","Skal Labissiere averaged 3 points in the regular season")),
			"maurice" | "mo" |"harkless"| "maurice harkless" | "mo harkless" => Ok(Response::new_simple("points","Mo Harkless averaged 7.7 points in the regular season")),
			"zach" | "collins" | "zach collins" => Ok(Response::new_simple("points","Zach Collins averaged 6.6 points in the regular season")),
			"anthony" | "simmons" | "anthony simmons" => Ok(Response::new_simple("points","Anfernee Simons averaged 3.8 points in the regular season")),
			"yusuf" | "yusuf ergin" | "ner berkovic" => Ok(Response::new_simple("points","Jusuf Nurkic averaged 15.6 points in the regular season")),
			"gary" | "gary trent junior" | "gary trent" => Ok(Response::new_simple("points","Gary Trent Junior averaged 2.7 points in the regular season")),
			"seth" | "curry" | "seth curry" => Ok(Response::new_simple("points","Seth Curry averaged 7.9 points in the regular season")),
			"rodney" | "hood" | "rodney hood" => Ok(Response::new_simple("points","Rodney Hood averaged 11.2 points in the regular season")),
			"enes" | "kanter" | "enes kanter" => Ok(Response::new_simple("points","Enes Kanter averaged 13.7 points in the regular season")),
			"the team" => Ok(Response::new_simple("points","The Blazers, as a team, averaged 10.2 points per player in the regular season")),	
			_ => Ok(Response::new_simple("points","I don't recognize that players name, please try again")),
		}
	}else{
		Ok(Response::new_simple("points","invalid"))
	}
			
}

//Handles when asked for the average number of assists
fn handle_assists(req: &Request) -> Result<Response,HandlerError> {

	if let Some(ref s) = req.slot_value("name") {
		match s.to_lowercase().as_str(){
			"lillard"|"damian"|"damian Lillard" => Ok(Response::new_simple("assists","Damian Lillard averaged 6.9 assists in the regular season")),
			"c.j."|"mccullum"|"c.j. mccullum" => Ok(Response::new_simple("assists","CJ McCollum averaged 3 assists in the regular season")),
			"al farouq"|"aminu"|"al farouq aminu" => Ok(Response::new_simple("assists","Al-Farouq Aminu averaged 1.3 assists in the regular season")),
			"evan" | "turner" | "evan turner" => Ok(Response::new_simple("assists","Evan Turner averaged 3.9 assists in the regular season")),
			"jake" | "layman" | "jake layman" => Ok(Response::new_simple("assists","Jake Layman averaged 0.7 assists in the regular season")),
			"myers"|"leonard"|"myers Leonard" => Ok(Response::new_simple("assists","Meyers Leonard averaged 1.2 assists per game in the regular season")),
			"scott" | "lagusse" | "scott lagusse" => Ok(Response::new_simple("assists","Skal Labissiere averaged 0.5 assists in the regular season")),
			"maurice" | "mo" |"harkless"| "maurice harkless" | "mo harkless" => Ok(Response::new_simple("assists","Mo Harkless averaged 1.2 assists in the regular season")),
			"zach" | "collins" | "zach collins" => Ok(Response::new_simple("assists","Zach Collins averaged 0.9 assists in the regular season")),
			"anthony" | "simmons" | "anthony simmons" => Ok(Response::new_simple("assists","Anfernee Simons averaged 0.7 assists in the regular season")),
			"yusuf" | "yusuf ergin" | "ner berkovic" => Ok(Response::new_simple("assists","Jusuf Nurkic averaged 3.2 assists in the regular season")),
			"gary" | "gary trent junior" | "gary trent" => Ok(Response::new_simple("assists","Gary Trent Junior averaged 0.3 assists in the regular season")),
			"seth" | "curry" | "seth curry" => Ok(Response::new_simple("assists","Seth Curry averaged 0.9 assists in the regular season")),
			"rodney" | "hood" | "rodney hood" => Ok(Response::new_simple("assists","Rodney Hood averaged 1.8 assists in the regular season")),
			"enes" | "kanter" | "enes kanter" => Ok(Response::new_simple("assists","Enes Kanter averaged 1.7 assists in the regular season")),
			"the team" => Ok(Response::new_simple("assists","The Blazers, as a team, averaged 1.88 assists per player in the regular season")),	
			_ => Ok(Response::new_simple("assists","I don't recognize that players name, please try again")),
		}
	}else{
		Ok(Response::new_simple("assists","invalid"))
	}
			
}

//Handles when asked for field goal percentage
fn handle_fg(req: &Request) -> Result<Response,HandlerError> {

	if let Some(ref s) = req.slot_value("name") {
		match s.to_lowercase().as_str(){
			"lillard"|"damian"|"damian Lillard" => Ok(Response::new_simple("fg","Damian Lillard averaged a 44.4 percent field goal percentage in the regular season")),
			"c.j."|"mccullum"|"c.j. mccullum" => Ok(Response::new_simple("fg","CJ McCollum averaged a 45.4 percent field goal percentage in the regular season")),
			"al farouq"|"aminu"|"al farouq aminu" => Ok(Response::new_simple("fg","Al-Farouq Aminu averaged a 43.3 percent field goal percentage in the regular season")),
			"evan" | "turner" | "evan turner" => Ok(Response::new_simple("fg","Evan Turner averaged a 46 percent field goal percentage in the regular season")),
			"jake" | "layman" | "jake layman" => Ok(Response::new_simple("fg","Jake Layman averaged 50.9 percent field goal percentage in the regular season")),
			"myers"|"leonard"|"myers Leonard" => Ok(Response::new_simple("fg","Meyers Leonard averaged 54.5 percent field goal percentage per game in the regular season")),
			"scott" | "lagusse" | "scott lagusse" => Ok(Response::new_simple("fg","Skal Labissiere averaged a 53.1 percent field goal percentage in the regular season")),
			"maurice" | "mo" |"harkless"| "maurice harkless" | "mo harkless" => Ok(Response::new_simple("fg","Mo Harkless averaged a 48.7 percent field goal percentage in the regular season")),
			"zach" | "collins" | "zach collins" => Ok(Response::new_simple("fg","Zach Collins averaged a 47.3 percent field goal percentage in the regular season")),
			"anthony" | "simmons" | "anthony simmons" => Ok(Response::new_simple("fg","Anfernee Simons averaged a 44.4 percent field goal percentage in the regular season")),
			"yusuf" | "yusuf ergin" | "ner berkovic" => Ok(Response::new_simple("fg","Jusuf Nurkic averaged a 50.8 percent field goal percentage in the regular season")),
			"gary" | "gary trent junior" | "gary trent" => Ok(Response::new_simple("fg","Gary Trent Junior averaged a 32 percent field goal percentage in the regular season")),
			"seth" | "curry" | "seth curry" => Ok(Response::new_simple("fg","Seth Curry averaged a 45.6 percent field goal percentage in the regular season")),
			"rodney" | "hood" | "rodney hood" => Ok(Response::new_simple("fg","Rodney Hood averaged a 43.5 percent field goal percentage in the regular season")),
			"enes" | "kanter" | "enes kanter" => Ok(Response::new_simple("fg","Enes Kanter averaged a 54.9 percent field goal percentage in the regular season")),
			"the team" => Ok(Response::new_simple("fg","The Blazers, as a team, averaged a 47.02 percent field goal percentage per player in the regular season")),	
			_ => Ok(Response::new_simple("fg","I don't recognize that players name, please try again")),
		}
	}else{
		Ok(Response::new_simple("fg","invalid"))
	}
			
}


//Handles when asked for three point percentage
fn handle_three(req: &Request) -> Result<Response,HandlerError> {

	if let Some(ref s) = req.slot_value("name") {
		match s.to_lowercase().as_str(){
			"lillard"|"damian"|"damian Lillard" => Ok(Response::new_simple("three","Damian Lillard averaged a 36.9 percent three point percentage in the regular season")),
			"c.j."|"mccullum"|"c.j. mccullum" => Ok(Response::new_simple("three","CJ McCollum averaged a 37.5 percent three point percentage in the regular season")),
			"al farouq"|"aminu"|"al farouq aminu" => Ok(Response::new_simple("three","Al-Farouq Aminu averaged a 34.3 percent three point percentage in the regular season")),
			"evan" | "turner" | "evan turner" => Ok(Response::new_simple("three","Evan Turner averaged a 21.2 percent three point percentage in the regular season")),
			"jake" | "layman" | "jake layman" => Ok(Response::new_simple("three","Jake Layman averaged a 32.6 percent three point percentage in the regular season")),
			"myers"|"leonard"|"myers Leonard" => Ok(Response::new_simple("three","Meyers Leonard averaged a 45 percent three point percentage per game in the regular season")),
			"scott" | "lagusse" | "scott lagusse" => Ok(Response::new_simple("three","Skal Labissiere averaged a 46.2 percent three point percentage in the regular season")),
			"maurice" | "mo" |"harkless"| "maurice harkless" | "mo harkless" => Ok(Response::new_simple("three","Mo Harkless averaged a 27.5 percent three point percentage in the regular season")),
			"zach" | "collins" | "zach collins" => Ok(Response::new_simple("three","Zach Collins averaged a 33.1 percent three point percentage in the regular season")),
			"anthony" | "simmons" | "anthony simmons" => Ok(Response::new_simple("three","Anfernee Simons averaged a 34.5 percent three point percentage in the regular season")),
			"yusuf" | "yusuf ergin" | "ner berkovic" => Ok(Response::new_simple("three","Jusuf Nurkic averaged a 10.3 percent three point percentage in the regular season")),
			"gary" | "gary trent junior" | "gary trent" => Ok(Response::new_simple("three","Gary Trent Junior averaged a 23.8 percent three point percentage in the regular season")),
			"seth" | "curry" | "seth curry" => Ok(Response::new_simple("three","Seth Curry averaged a 45 percent three point percentage in the regular season")),
			"rodney" | "hood" | "rodney hood" => Ok(Response::new_simple("three","Rodney Hood averaged a 35.6 percent three point percentage in the regular season")),
			"enes" | "kanter" | "enes kanter" => Ok(Response::new_simple("three","Enes Kanter averaged a 29.4 percent three point percentage in the regular season")),
			"the team" => Ok(Response::new_simple("three","The Blazers, as a team, averaged a 32.86 percent three point percentage per player in the regular season")),	
			_ => Ok(Response::new_simple("three","I don't recognize that players name, please try again")),
		}
	}else{
		Ok(Response::new_simple("three","invalid"))
	}
			
}

//Handles when asked for free throw percentage
fn handle_ft(req: &Request) -> Result<Response,HandlerError> {

	if let Some(ref s) = req.slot_value("name") {
		match s.to_lowercase().as_str(){
			"lillard"|"damian"|"damian Lillard" => Ok(Response::new_simple("ft","Damian Lillard averaged a 91.2 percent free throw percentage in the regular season")),
			"c.j."|"mccullum"|"c.j. mccullum" => Ok(Response::new_simple("ft","CJ McCollum averaged an 82.8 percent free throw percentage in the regular season")),
			"al farouq"|"aminu"|"al farouq aminu" => Ok(Response::new_simple("ft","Al-Farouq Aminu averaged an 86.7 percent free throw percentage in the regular season")),
			"evan" | "turner" | "evan turner" => Ok(Response::new_simple("ft","Evan Turner averaged a 70.8 percent free throw percentage in the regular season")),
			"jake" | "layman" | "jake layman" => Ok(Response::new_simple("ft","Jake Layman averaged a 70.4 percent free throw percentage in the regular season")),
			"myers"|"leonard"|"myers Leonard" => Ok(Response::new_simple("ft","Meyers Leonard averaged an 84.3 percent free throw percentage per game in the regular season")),
			"scott" | "lagusse" | "scott lagusse" => Ok(Response::new_simple("ft","Skal Labissiere averaged a 52.9 percent free throw percentage in the regular season")),
			"maurice" | "mo" |"harkless"| "maurice harkless" | "mo harkless" => Ok(Response::new_simple("ft","Mo Harkless averaged a 67.1 percent free throw percentage in the regular season")),
			"zach" | "collins" | "zach collins" => Ok(Response::new_simple("ft","Zach Collins averaged a 74.6 percent free throw percentage in the regular season")),
			"anthony" | "simmons" | "anthony simmons" => Ok(Response::new_simple("ft","Anfernee Simons averaged a 56.3 percent free throw percentage in the regular season")),
			"yusuf" | "yusuf ergin" | "ner berkovic" => Ok(Response::new_simple("ft","Jusuf Nurkic averaged a 77.3 percent free throw percentage in the regular season")),
			"gary" | "gary trent junior" | "gary trent" => Ok(Response::new_simple("ft","Gary Trent Junior averaged a 42.9 percent free throw percentage in the regular season")),
			"seth" | "curry" | "seth curry" => Ok(Response::new_simple("ft","Seth Curry averaged an 84.6 percent free throw percentage in the regular season")),
			"rodney" | "hood" | "rodney hood" => Ok(Response::new_simple("ft","Rodney Hood averaged an 88.4 percent free throw percentage in the regular season")),
			"enes" | "kanter" | "enes kanter" => Ok(Response::new_simple("ft","Enes Kanter averaged a 78.7 percent free throw percentage in the regular season")),
			"the team" => Ok(Response::new_simple("ft","The Blazers, as a team, averaged a 73.93 percent free throw percentage per player in the regular season")),	
			_ => Ok(Response::new_simple("ft","I don't recognize that players name, please try again")),
		}
	}else{
		Ok(Response::new_simple("ft","invalid"))
	}
			
}

//Lets the user know that Damian Lillard is Russell Westbrooks father
fn handle_baby() -> Result<Response,HandlerError>{
	
	Ok(Response::new_simple("baby", "Damian Lillard rocks Russell Westbrook to sleep"))

}

//Handles for invalid questions
fn handle_invalid() -> Result<Response,HandlerError>{
	
	Ok(Response::new_simple("invalid", "That question is not valid please try something different"))

}

//thank you handler
fn handle_thanks() -> Result<Response,HandlerError>{

	Ok(Response::new_simple("thanks", "Josh, the creator of this skill, would like to give a special thanks to his wife Tara and his children Brayden and Charlie. I love you"))

}

//This is the full handler that deals with all the possible user intents.  The first two are built in Alexa intents and the final
//match statement deals with all the intents I generated
fn my_handler(req: Request, _ctx: Context) -> Result<Response,HandlerError> {

    match req.intent() {
        IntentType::Help => handle_help(&req),
		IntentType::User(name) => {
			match name.as_str() {
				"games" => handle_games(&req),
				"rebounds" => handle_rbs(&req),
				"assists" => handle_assists(&req),
				"points" => handle_points(&req),
				"fg" => handle_fg(&req),
				"three" => handle_three(&req),
				"ft" => handle_ft(&req),
				"baby" => handle_baby(),
				"thanks" => handle_thanks(),
				_ => handle_invalid(),
			}
		},
        _ => handle_cancel (&req),
    }
}

//Invokes the AWS Lambda
fn main() -> Result<(), Box<dyn Error>> {

    lambda!(my_handler);

    Ok(())
}
