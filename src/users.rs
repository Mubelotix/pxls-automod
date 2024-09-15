use std::collections::HashMap;
use std::sync::LazyLock;

pub static USERS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("abdoul_aziz_sadio", "ITI"),
        ("adam_boufous", "GM"),
        ("adeline_royet", "GM"),
        ("adham_hamoud", "MRIE"),
        ("adrien_his", "GM"),
        ("agathe_goudigan", "CFI"),
        ("alexandre_clergeaud-martinez", "GM"),
        ("alexandre_dos_santos", "EP"),
        ("alexandre_quide", "MRIE"),
        ("alexane_coursieres", "EP"),
        ("alexane_ducos", "CFI"),
        ("alexia_durand", "MRIE"),
        ("ali_benadjel", "MRIE"),
        ("ali_ikched", "ITI"),
        ("alicia_evaristo", "CFI"),
        ("alkan_polat", "MRIE"),
        ("alla_seck", "GC"),
        ("amaury_bailly", "EP"),
        ("ambroise_arrigoni", "ITI"),
        ("amelie_mathieu", "GM"),
        ("amelie_mauduit", "MRIE"),
        ("ameline_turpin", "CFI"),
        ("ami_colle_mbaye", "MECA"),
        ("amin_oubassour", "EP"),
        ("amr_zaki_salih", "GM"),
        ("amsatou_diop", "ITI"),
        ("anaelle_lhostis", "EP"),
        ("anais_guellaff", "MECA"),
        ("anass_yomni", "ITI"),
        ("anastasia_laudet--shablinskaya", "GM"),
        ("angele_cahu", "CFI"),
        ("angele_cotelle", "MECA"),
        ("anne_dutrieux", "MRIE"),
        ("annelyse_faujouron", "GM"),
        ("anouk_petitgas", "GC"),
        ("anta_ndiongue", "EP"),
        ("anthony_rizk", "EP"),
        ("antoine_berdin", "CFI"),
        ("antoine_fernandes", "EP"),
        ("antoine_plu", "CFI"),
        ("antonin_tual--boutet", "CFI"),
        ("apolline_desmorat", "GM"),
        ("ariel_tingaud", "MECA"),
        ("armand_burtin", "EP"),
        ("armand_le_borgne", "EP"),
        ("arthur_lang", "MECA"),
        ("arthur_lefebvre01", "MECA"),
        ("arthur_moucquot", "MECA"),
        ("arthur_sarrau", "GM"),
        ("arthur_voegele", "EP"),
        ("aubin_ternisien", "MECA"),
        ("aude_illy", "EP"),
        ("audran_lauvergeat", "GC"),
        ("augustin_bressac", "ITI"),
        ("aurore_garnier", "MRIE"),
        ("axel_desouche", "EP"),
        ("axel_filliette", "EP"),
        ("axel_joubioux", "MECA"),
        ("axel_martin", "ITI"),
        ("ayoub_tabrich", "EP"),
        ("azara_traore", "CFI"),
        ("baptiste_genoudet", "MECA"),
        ("baptiste_muller", "GM"),
        ("baptiste_neckebrock", "CFI"),
        ("bastian_portemann", "EP"),
        ("benoit_cernay", "MECA"),
        ("bien-aime_bounkita_ndzoumi", "MECA"),
        ("bilal_bammou", "MRIE"),
        ("brieuc_tastard", "EP"),
        ("camille_audoin", "GM"),
        ("camille_beux", "EP"),
        ("camille_borges", "GC"),
        ("camille_genin", "MECA"),
        ("candice_broyer", "GM"),
        ("candice_delfosse", "MRIE"),
        ("cassandre_debure", "CFI"),
        ("cecile_gardeil", "EP"),
        ("celia_glinel", "GM"),
        ("celian_bellier", "GC"),
        ("celian_valentin", "EP"),
        ("celyan_tostain", "EP"),
        ("charlie_kerhomen", "CFI"),
        ("charlotte_hillairet", "CFI"),
        ("cheikh_seck", "GM"),
        ("chloe_fraimbault", "ITI"),
        ("chloe_granier", "CFI"),
        ("chloe_mercier", "EP"),
        ("chloe_senecal", "MRIE"),
        ("christian_andriamialy", "ITI"),
        ("christina_taty_isael", "MRIE"),
        ("christopher_speer", "CFI"),
        ("clara_guittet", "EP"),
        ("clara_madec", "GC"),
        ("clara_meline", "EP"),
        ("clarisse_ghesquiere-dierickx", "GC"),
        ("clarisse_lemaire", "MECA"),
        ("clarisse_raingeard", "EP"),
        ("clemence_gastin", "MRIE"),
        ("clement_auclair", "CFI"),
        ("clement_nitschko", "MECA"),
        ("colin_puech", "MECA"),
        ("coline_martinez", "GC"),
        ("corentin_pidol", "EP"),
        ("damien_pinto", "EP"),
        ("damien_simeon", "GC"),
        ("daniel_viard", "MRIE"),
        ("darius_nazeri", "ITI"),
        ("djivaalakshmy_dhanasekaran", "CFI"),
        ("dylan_sanson", "ITI"),
        ("el_hadji_dieme", "EP"),
        ("elias_chataigner", "MECA"),
        ("elias_le_calvez", "MRIE"),
        ("elisa_picou", "EP"),
        ("elora_salvignol", "GC"),
        ("elouen_roturier", "MECA"),
        ("elsa_varin", "CFI"),
        ("emilie_guignon", "CFI"),
        ("emily_mekireche", "MECA"),
        ("emma_beuriot", "MECA"),
        ("emma_reymondoux", "CFI"),
        ("emmanuel_charrier", "ITI"),
        ("emmy_vadon", "EP"),
        ("enzo_ben_saad", "GC"),
        ("enzo_coeffec", "CFI"),
        ("enzo_danican", "MRIE"),
        ("enzo_trohel", "MECA"),
        ("esteban_ayora-guia", "CFI"),
        ("ethan_arnaud", "MECA"),
        ("ethan_elfassi", "ITI"),
        ("evan_le_nouy", "EP"),
        ("evan_lefevre", "MECA"),
        ("fatimata_teliko", "GM"),
        ("flavien_madelon", "MECA"),
        ("florent_tariolle", "ITI"),
        ("floriane_georges", "MRIE"),
        ("florise_ruet", "CFI"),
        ("fouad_bellili", "ITI"),
        ("gabin_chanteloup", "MECA"),
        ("gabin_lemoisson", "EP"),
        ("gabriel_brejaud", "MECA"),
        ("gabriel_faisnel", "EP"),
        ("garance_godon", "MECA"),
        ("garmi_ndiaye", "MRIE"),
        ("gatien_dechamps", "GM"),
        ("gauvain_noiton", "ITI"),
        ("giovanna_brun", "CFI"),
        ("hippolyte_perraud", "GM"),
        ("hugo_legras", "MECA"),
        ("hugo_paul", "EP"),
        ("ibrahima_diallo01", "CFI"),
        ("ilan_brinkert", "EP"),
        ("iman_er_rami_sahrai", "MECA"),
        ("imane_raghib", "MRIE"),
        ("ines_chastel", "MRIE"),
        ("iris_dussuyer", "ITI"),
        ("ismail_el_guir", "GM"),
        ("jack_su", "ITI"),
        ("jade_brochard", "ITI"),
        ("jean_hallot", "GM"),
        ("jeanne_bourges", "MRIE"),
        ("jeanne_marot", "EP"),
        ("jenifer_vassaux", "CFI"),
        ("jihad_samaha", "MECA"),
        ("joris_moisan", "MRIE"),
        ("jules_flament", "MRIE"),
        ("jules_galhardo", "ITI"),
        ("julianne_droulout", "CFI"),
        ("julie_hostier", "CFI"),
        ("juliette_barou", "EP"),
        ("juliette_noury", "GC"),
        ("kadiata_abou_sow", "GC"),
        ("kangping_huang", "GM"),
        ("kawtar_el_gueddari", "ITI"),
        ("kenza_bendouro", "EP"),
        ("kezia_garrigues", "MRIE"),
        ("lamiaa_bentlib", "GC"),
        ("latifa_obone_mba", "MRIE"),
        ("laura_beyron", "MRIE"),
        ("laura_robert", "MECA"),
        ("laure_malnoux", "GC"),
        ("laurene_lecomte", "GM"),
        ("laurine_fortin", "ITI"),
        ("lea_aurand", "EP"),
        ("lea_daniel", "GM"),
        ("leo_lacroix", "CFI"),
        ("lila_bertrand-adam", "EP"),
        ("lilou_bonche", "MECA"),
        ("lina_el_hachmi", "GM"),
        ("lina_el_omari_bouya", "ITI"),
        ("lisa_bonnet-maeda", "MRIE"),
        ("loic_seba", "GM"),
        ("lorenzo_meche", "MRIE"),
        ("lorianne_fuster", "GC"),
        ("loris_el_hadri", "MRIE"),
        ("louis_anne", "ITI"),
        ("louis_hedin", "MECA"),
        ("louis_lenoble", "ITI"),
        ("louise_feunteun", "EP"),
        ("louise_laisney", "MRIE"),
        ("louison_frappe", "MECA"),
        ("luc_beard", "ITI"),
        ("luca_ponzano_andrade", "EP"),
        ("lucas_leblond", "MECA"),
        ("lucas_lefebvre", "GM"),
        ("lucien_cousseau", "MECA"),
        ("lucien_quemener", "MRIE"),
        ("ludivine_delalleau", "GC"),
        ("ludivine_gonzalez", "EP"),
        ("luiza_mattedi", "ITI"),
        ("lyna_andre", "CFI"),
        ("lynn_el_khoury", "MECA"),
        ("madeleine_l_hoist", "GC"),
        ("mael_planchot", "ITI"),
        ("mael_verdenal", "GC"),
        ("maia_tribout", "CFI"),
        ("mame_dieguene_diouf", "GM"),
        ("manon_bienfait", "EP"),
        ("marc-antoine_carlotti", "GC"),
        ("marc_azzi", "CFI"),
        ("margaux_guillaumet", "GC"),
        ("mariana_silva_pinto01", "MRIE"),
        ("marie-pier_piche", "MECA"),
        ("marie_brument", "CFI"),
        ("marie_hellequin", "EP"),
        ("marie_philippe", "EP"),
        ("marine_crochemore", "CFI"),
        ("marion_tiphaigne", "MECA"),
        ("marius_landreau", "GM"),
        ("martin_doyen", "MECA"),
        ("martin_eschalier", "GM"),
        ("mateo_maguer", "MECA"),
        ("matheo_desfeux", "MECA"),
        ("matheo_tran", "MECA"),
        ("mathian_prigent", "MECA"),
        ("mathieu_lemaguerou", "EP"),
        ("mathilde_billon", "GC"),
        ("mathilde_dore", "CFI"),
        ("mathilde_hiesse", "GC"),
        ("mathilde_rascoussier", "CFI"),
        ("mathilde_tanquerel", "ITI"),
        ("mathis_arcega", "ITI"),
        ("mathis_aubry", "MRIE"),
        ("mathis_bole_feysot", "ITI"),
        ("mathis_muller", "MECA"),
        ("mathis_troboe", "EP"),
        ("matteo_queval", "MRIE"),
        ("max_feldman--le_meur", "CFI"),
        ("maxime_delair", "GC"),
        ("maxime_etienne", "ITI"),
        ("maxime_jouanneau", "ITI"),
        ("maxime_lamy", "CFI"),
        ("maxime_zaccardo", "MECA"),
        ("maxine_rossignol", "CFI"),
        ("melanie_duret", "MRIE"),
        ("merveille_togbe", "GC"),
        ("michel_vespier", "ITI"),
        ("moehau_dufrene", "ITI"),
        ("mohamed-rayen_ben_abderrahmane", "ITI"),
        ("mohamed_el_masaoudi", "GC"),
        ("mohamed_laghmadi", "CFI"),
        ("mohamed_tauil", "ITI"),
        ("molka_mekni", "GM"),
        ("morgane_andre", "CFI"),
        ("mouad_sheradj-drissi", "GM"),
        ("nader_el_ghoche", "EP"),
        ("nael_mathlouthi", "CFI"),
        ("natan_massot", "GM"),
        ("nathan_boutigny", "MECA"),
        ("nathan_graff", "MECA"),
        ("nathan_gros", "MECA"),
        ("nathan_levasseur", "ITI"),
        ("nathan_mercier", "GM"),
        ("nathan_sourdrille", "ITI"),
        ("nayis-denis_said_ali", "GC"),
        ("neil_huby", "GM"),
        ("nicolas_gilletta", "MECA"),
        ("nicolas_guindron", "GC"),
        ("nidale_fawzi", "GM"),
        ("nils_girault", "GC"),
        ("nina_knoepffler", "GM"),
        ("nina_zeddoun", "GC"),
        ("ninon_legendre", "CFI"),
        ("nizar_duqueh", "ITI"),
        ("noah_feneuille", "MECA"),
        ("noe_camus", "MECA"),
        ("noe_laruelle", "GM"),
        ("norah_vigouroux", "EP"),
        ("nour_chaabane", "GM"),
        ("oceane_remy", "MRIE"),
        ("orlane_montebran", "EP"),
        ("paul_barbieux", "EP"),
        ("paul_favery", "CFI"),
        ("paul_fougeray", "ITI"),
        ("paul_kerfant", "ITI"),
        ("paul_lozach", "GM"),
        ("pauline_gourlain", "MECA"),
        ("phybie_percebois", "EP"),
        ("pierre_lynch", "MECA"),
        ("quentin_menguy", "EP"),
        ("quentin_meuleman", "MECA"),
        ("quentin_sellal", "EP"),
        ("quentin_vermeulen", "MECA"),
        ("quynh_chi_pham", "CFI"),
        ("ranim_bouaicha", "CFI"),
        ("raphael_cadas", "MECA"),
        ("raphael_daligaux", "EP"),
        ("raphael_djaroun", "EP"),
        ("raphael_evain", "GC"),
        ("raphael_flamand", "EP"),
        ("raphael_fourneaux", "MECA"),
        ("raphael_guesneux", "EP"),
        ("raphael_roger", "ITI"),
        ("raphael_schneider", "GM"),
        ("raphael_senellart", "ITI"),
        ("raphaelle_pichot", "MRIE"),
        ("rayen_zouaghi", "ITI"),
        ("reda_bouzid", "ITI"),
        ("remi_debernardi", "MECA"),
        ("remi_rozier", "MRIE"),
        ("rihab_kattat", "CFI"),
        ("rizlene_damni", "MRIE"),
        ("robin_bousquet--chtepenko", "MECA"),
        ("robin_gouard", "CFI"),
        ("robin_pinel", "GM"),
        ("robin_renaux", "MECA"),
        ("rojan_kaya", "ITI"),
        ("romain_jolly", "EP"),
        ("romain_steffen", "GM"),
        ("romane_gilloots", "EP"),
        ("romane_laneres", "MRIE"),
        ("romane_ledru", "EP"),
        ("romane_noublanche", "EP"),
        ("romann_bonhomme", "ITI"),
        ("ryann_dera-schtyk", "MRIE"),
        ("sacha_gerard", "EP"),
        ("salah_eddine_el_asraoui", "GC"),
        ("salma_el_mansouri", "GM"),
        ("salome_puig", "GM"),
        ("salomon_allais", "EP"),
        ("sam_thibault", "EP"),
        ("samuel_bloomfield", "CFI"),
        ("samuel_canada", "GC"),
        ("samuel_merieult", "MECA"),
        ("sanae_bouhamdi", "CFI"),
        ("sandra_elhajj", "CFI"),
        ("sarah_benslimane", "ITI"),
        ("selma_douagi", "ITI"),
        ("sena_lebourgeois", "GC"),
        ("siman_amahmid", "CFI"),
        ("simon_chopin", "EP"),
        ("sofien_fiess", "MECA"),
        ("solenn_hubert", "MECA"),
        ("soungalo_diabate", "GC"),
        ("stanislas_marie", "MECA"),
        ("teano_guyonnet", "GM"),
        ("theo_dujardin", "MRIE"),
        ("theo_ferron", "EP"),
        ("theo_hubert", "MECA"),
        ("theo_lynch", "MECA"),
        ("theo_sefrioui", "EP"),
        ("theotime_cadoret", "EP"),
        ("theotime_sedilot", "EP"),
        ("thibault_henry", "MECA"),
        ("thibaut_frebourg", "GC"),
        ("thomas_bavoux", "ITI"),
        ("thomas_bonneviale", "ITI"),
        ("thomas_corvina", "GC"),
        ("thomas_demeules", "GM"),
        ("thomas_eveno", "EP"),
        ("thomas_fournier", "MRIE"),
        ("thomas_guillot", "GM"),
        ("thomas_josse", "MECA"),
        ("thomas_jouans", "GC"),
        ("tim_dhien", "EP"),
        ("timothee_dupont", "MRIE"),
        ("timothee_lawani", "MECA"),
        ("tiphaine_crevenat", "MECA"),
        ("tiphaine_richet", "CFI"),
        ("titouan_darrigan", "GC"),
        ("titouan_georges", "MECA"),
        ("tom_berthelot", "GC"),
        ("tom_collet", "EP"),
        ("tom_guenot", "ITI"),
        ("tom_lhuguet", "MECA"),
        ("tom_martin_laprade", "MECA"),
        ("tom_philippe", "GC"),
        ("tristan_legras", "MRIE"),
        ("tristan_richard", "EP"),
        ("tristan_torteau", "MECA"),
        ("tuan_anh_le", "GM"),
        ("ugo_lartigau", "GC"),
        ("valentin_braconnot", "MECA"),
        ("vezone_ngoyi_pemba", "MRIE"),
        ("victor_dubuc", "MECA"),
        ("victoria_blanca", "CFI"),
        ("victorien_merand--maurel", "MRIE"),
        ("virgil_ducrocq", "GM"),
        ("william_bossard", "CFI"),
        ("william_listemann", "EP"),
        ("yann-mathis_delahaye", "GM"),
        ("yasser_hinane", "GM"),
        ("yassine_mhiri", "GM"),
        ("yassine_moumsik", "GM"),
        ("yixuan_zhi", "CFI"),
        ("yohann_delaplace", "ITI"),
        ("yohann_guenegou", "ITI"),
        ("younes_el_asri", "GM"),
        ("youssra_chellali", "MRIE"),
        ("yuzhen_ni", "ITI"),
        ("zakaria_barhoumi", "ITI"),
    ])
});
