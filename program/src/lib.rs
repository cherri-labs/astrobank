/*******************************************************
   ------------------------------------------------
     ___         __                ____              __  
    /   |  _____/ /__________     / __ )____ _____  / /__
   / /| | / ___/ __/ ___/ __ \   / __  / __ `/ __ \/ //_/
  / ___ |(__  / /_/ /  / /_/ /  / /_/ / /_/ / / / / ,<   
 /_/  |_/____/\__/_/   \____/  /_____/\__,_/_/ /_/_/|_|  
   ----  v.0.4.0  ----------   -----------------------

 ******************************************************

 Astro Gate program built on Solana to lock and
 distribute tokens.

 ******************************************************

 For any help or additional support you can join our
  Discord: https://discord.gg/R7bY7DQAWz/
  Twitter: https://twitter.com/LASRNFT/

 More info on: https:/www.astronautsnft.xyz/

 ******************************************************/

mod owner_token;
mod accounts;
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
