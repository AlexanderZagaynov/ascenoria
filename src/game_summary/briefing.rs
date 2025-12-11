//! Mission briefing text generation.

/// Generates a mission briefing based on the species.
pub fn generate_mission_briefing(species_name: &str, species_id: &str) -> String {
    // Mission briefing varies by species personality
    match species_id {
        "minions" => format!(
            "The {} have emerged from their homeworld with one supreme purpose: galactic domination through any means necessary. \
            Your cunning and resourcefulness will be tested as you navigate the treacherous waters of interstellar politics.\n\n\
            Build your empire. Crush your enemies. The galaxy awaits its new masters.",
            species_name
        ),
        "chamachies" => format!(
            "The noble {} embark upon their greatest journey yet. Guided by ancient traditions and unwavering honor, \
            your people seek to bring enlightenment to the cosmos.\n\n\
            Forge alliances with worthy species. Expand your influence through wisdom and strength. The stars call to your destiny.",
            species_name
        ),
        "orfa" => format!(
            "The mysterious {} venture forth from the depths of their ocean world. \
            Your unique perspective and adaptability will prove invaluable in the harsh vacuum of space.\n\n\
            Explore strange new worlds. Discover the secrets of the cosmos. Your journey into the unknown begins now.",
            species_name
        ),
        "govorom" => format!(
            "The ancient {} have awakened after millennia of contemplation. \
            With vast knowledge accumulated over eons, your species now turns its gaze to the stars.\n\n\
            Share your wisdom or guard it jealously. The choice is yours. The universe trembles at your awakening.",
            species_name
        ),
        "saurians" => format!(
            "The mighty {} march forth to claim their rightful place among the stars. \
            Proud warriors and fierce competitors, your people know only victory.\n\n\
            Conquer. Dominate. Rule. The weak shall bow before the strong.",
            species_name
        ),
        "arbryls" => format!(
            "The enigmatic {} spread their tendrils across the galaxy. \
            Patient and methodical, your species plays the long game in the cosmic struggle for supremacy.\n\n\
            Grow. Adapt. Consume. The galaxy is but fertile ground for your kind.",
            species_name
        ),
        "frutmaka" => format!(
            "The industrious {} set forth to build a new future among the stars. \
            Masters of technology and engineering, no challenge is too great for your ingenuity.\n\n\
            Create. Innovate. Construct. The galaxy shall know your works.",
            species_name
        ),
        "shevar" => format!(
            "The ethereal {} drift through the cosmos seeking harmony and balance. \
            Your connection to the fundamental forces of the universe grants unique insight.\n\n\
            Seek balance. Find harmony. Bring peace to a chaotic galaxy... or not.",
            species_name
        ),
        "dubtaks" => format!(
            "The resilient {} emerge from their harsh homeworld ready for anything. \
            Survivors by nature, your people thrive where others would perish.\n\n\
            Endure. Persist. Overcome. The galaxy's challenges are nothing compared to home.",
            species_name
        ),
        _ => format!(
            "The {} embark on an epic journey to the stars. \
            A new chapter in your species' history begins today.\n\n\
            Explore the cosmos. Build your civilization. \
            Write your legend among the stars.",
            species_name
        ),
    }
}
