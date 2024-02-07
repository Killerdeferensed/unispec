use nalgebra::{zero, DMatrix, DVector};

pub fn get_background(data: &[u32], order: u8) -> Vec<f64> {
    // Hardcore linear algebra porn
    let mut mu: u8 = 0;
    let mut n: u8 = 1;
    let mut m: u8 = 0;
    let mut f: u8 = 0;
    let mut ud: bool = false;
    let l = data.len();
    let mut E: Vec<f64> = vec![];

    let y = DVector::from_iterator(l, data.into_iter().map(|x| f64::from(*x))); // data vector
    let x = DVector::from_iterator(
        l,
        (0..u32::try_from(l).expect("There should be less than 4bil points, lol"))
            .into_iter()
            .map(f64::from),
    ); // indexes vector

    let blank_p = {
        let mut tmp: DMatrix<f64> = DMatrix::<f64>::zeros({ order + 1 }.into(), data.len());
        tmp.fill_row(0, 1.0);
        tmp.fill_row(1, 1.0);
        tmp
    }; // blank matrix (for cloning)
    let blank_v = DVector::<f64>::zeros(order.into()); // blank vector (for cloning)

    let mut w = vec![y.clone()]; //weigts (direct currently)
    let mut p = vec![blank_p.clone()]; // polynomials matrix
    let mut gamma: Vec<DVector<f64>> = vec![];
    fn gamma_build_next(w: &Vec<DVector<f64>>, mu: u8, gamma: &mut Vec<DVector<f64>>, order: u8) { // blank gamma vector building function
        // println!("Starting building gamma blanket");
        let first = DMatrix::<f64>::from_diagonal(&w[mu as usize])
            .pseudo_inverse(1e-6)
            .expect("Should be able to pseudo inverse gamma")
            .trace();
        // println!("Built first enement\n{}", first);
        let mut vec = DVector::<f64>::zeros(order as usize);
        // println!("Built vec element\n{}", vec);
        vec[(0, 0)] = first;
        // println!("Changed vec element\n{}", vec);
        gamma.push(vec);
        // println!("Pushed vec to gamma \n{}", gamma[{mu} as usize]);
}

    let mut sigma = vec![blank_v.clone()]; // Patrick Batemann
// ::::::::::::::::::::::::::::::::::::------------===============---:----======+++++++***************************************************
// :::::::::::::::::::::::::::::::::--------------===========--:..:=*###*=-===-::--=++++**************************************************
// ::::::::::::::::::::::::::::::::::--------------===-=---:...-=+#%%%%%%%%%%%%##*+++=-=+++***********************************************
// :::::::::::::::::::::::::::::::::--------------------::=+*#%%%%%%%%%%%@@%%%%%%####%#*===+++********************************************
// :::::::::::::::::::::::::::::::::----------------:...:=*%%%%%%%%%@@%%@@@@@%%%#%####%%#*+=::-+++****************************************
// ::::::::::::::::::::::::::::::::::-------------:.-*%%%%%%%%%%%%%%%@@@@@%@%@%%###***##*+**#+::-=+++*************************************
// :::::::::::::::::::::::::::::::::-------------=*%@@%@%%%%%%%%%@@@@@@@@%%%%%%%#**++=+*#+=+#%*+-..-=++***********************************
// ::::::::::::::::::::::::::::::::------------=#%%@@@@@@%%%%%%%%%@@@@@@@@%%%%%%#*++=++=+*+-*#***=-====++*********************************
// ::::::::::::::::::::::::::::::::-----------=#%@@@@@@@@%%%%%%%%%%@@@@@@@@@%%%%%###*+#==##=*+#*+==+++:==+********************************
// ::::::::::::::::::::::::::::::::--------:-+#%@@@@@@@@@@%%%%%%%%%%%%%%%%%%%%%%%%%%##%+*###**#*+-*=====-=+*******************************
// ::::::::::::::::::::::::::::::::------::-+#%%@@@@@@%%%%%#####**####%%####%%%%%%%#########%###**#*=-----=+******************************
// :::::::::::::::::::::::::::::::::::--:::=+%%@@@@@@%%%%##**++++++****############**++++****#######**++=--=+*****************************
// ::::::::::::::::::::::::::::::::::::::-*#%%%%%@@@%%%##**=-==++++******#####***++==----::--=+**######*=---=+****************************
// ::::::::::::::::::::::::::::::::::::--=#%%%%%%@@%%##*++=--===++++***#*******++=====---::....:-+**####*+-=-+****************************
// ::::::::::::::::::::::::::::::::::=*++##%%%%@@@%%#*=========+++********+++++=========-::.....:-+*****+=====+***************************
// ::::::::::::::::::::::::::::::::-*##=+#%#%%@@%%%#*+=:-=====++++********++****+++======::.......:=**++===+==+***************************
// :::::::::::::::::::::::::::::::--++=*##%#%%%%%%##*=--=====+++++***********++++*****+=::.........--++=-=+===++**************************
// :::::::::::::::::::::::::::::::::=++*%##%%%%%%%#*+=-========++*********************==-..........:-=+=--=+==+***************************
// ::::::::::::::::::::::::::::::::-+*###%%%%%%%%#*++=====++++++++++*****###******##*+++=:..........-+++======+***************************
// ::::::::::::::::::::::::::::::::+%%%%%%%######*+=======++++++++***#########****##**+=:--.........-+%#+++==+++**************************
// ::::::::::::::::::::::::::::::::#%%%%%%%%%%##**++===-==+++++**#######*****##*******+++-..........:=*##*+==++***************************
// ::::::::::::::::::::::::::::::::#%%%%%%%%%%##**++===-=+**###%%%%%%%%#*************+++=-:.........:-*%#*+==++***************************
// ::::::::::::::::::::::::::::::::=%%%%%%%%%%##*+======*###%%%%%%%%%%%%##********######*+:.........:-+##*+-+++***************************
// ::::::::::::::::::::::::::::::::+%%%%%%%%%%#*==-===*#######****#*######********###%%%%##=:.......:==*#*+=+++***************************
// :::::::::::::::::::::::::::::::-*%%%%%%%%%%#=---==+*###############*#*#********######%##*=-:.....:=####*==+++**************************
// ::::::::::::::::::::::::::::::-*#%%%%%%%%%#+-----=**####%%%%%%%%%%####***************####*=--:...:+####*==++***************************
// ::::::::::::::::::::::::::::::*%##*#%%%%%#*=-----+*##%%%%%%@@@@%%%%%###************###*##**++=-:.=##***===+++**************************
// :::::::::::::::::::::::::::::::+##=++%%%%*+=----=*##%%%#%%%%%%%%%%%%%###****+**##%%%%%%%%#***+-::*%##*+==+++++*************************
// ::::::::::::::::::::::::::::::::*+:==#%%**+----=+***###*#####%%%%%%%%#####*=-+#%%%%#%%%%%###*+-.:#%%#====+++++*************************
// ::::::::::::::::::::::::::::::::+=*#####*+==--===+***##%####%%%%%%%%######*:.-*#%%%%%#*++*##**=.:*%%+-==++++++*************************
// ::::::::::::::::::::::::::::::::-+#%#+-**+==--==+++***#%%%%%%%%#%######****-...*######*+-:.+*=:.:-+=:-=+++++++*++**********************
// :::::::::::::::::::::::::::::::::+##*---*+=======+=+**##############***++**=:...=*#####*=-=-:...:--=-=+++++++++++**+*******************
// :::::::::::::::::::::::::::::::::===+===-==========+**##########*++++++===*+:.....::-=+*====::..:---==++++++++++++++*+*+***************
// :::::::::::::::::::::::::::::::::--=-++=-==========+**#########**++++==--=++:......:------:.::..:=-==+++++++++++++++++*+*+*************
// ::::::::::::::::::::::::::::::::::-=++*+-==========+**###########*====--=+*+-...::..:-=--::.....==-=+++++++++++++++++++++++++++********
// :::::::::::::::::::::::::::::::::-===+**=====+==+++***##########*--=+=--=+*+-...:+*=----=:......--==++++++++++++++++++++++++++*++++****
// ::::::::::::::::::::::::::::::::::+***##*===+++++++****#########*+++++==+***=:...=##**+=--......-====+++++++++++++++++++++++++++++++++*
// ::::::::::::::::::::::::::::::::::+*#%%%*===++++++*****#############%%#++***+::--*#####*+=-::..:-=====+++++++++++++++++++++++++++++++++
// ::::::::::::::::::::::::::::::::::+*#%%@#===++++*****##################*+*#*-::-**######*+-:::.:-=====+=+++++++++++++++++++++++++++++++
// ::::::::::::::::::::::::::::::::::=*##%%%===+++**++*####################**=-::::**#######*=-::.:==========+=+++++++++++++++++++++++++++
// ::::::::::::::::::::::::::::::::::-**#%@%===+++**+**######*##*############*+--::+**######*----:-===================++++++++++++++++++++
// :::::::::::::::::::::::::::::::::::*##%%%+==+++**+**###%%%##*#**##########*+=-:::-+*#####+::=::-=====================+=++++++++++++++++
// :::::::::::::::::::::::::::::::::::-*%%%%*==+++**+#######%%%%##+**########**+====-=*#####+:=-:--=================================+====+
// ::::::::::::::::::::::::::::::::::::=##%%*==++++***####***##%%#++++*+++++++--=#%@%%#####*-==::-========================================
// :::::::::::::::::::::::::::::::::::::+#%%#===+++****###*****####%%@%#***++=:-++-::.-*###==-::---=======================================
// ::::::::::::::::::::::::::::::::::::::+%%#==+++++****##********#####+++++===-:......*##+--::---------==================================
// :::::::::::::::::::::::::::::::::::::::*##==++++++**+*##*********######**+=--:::...:**=::::-----------------==--=-=-=======-========-==
// :::::::::::::::::::::::::::::::::::::::-##===+++++++++*#*********########**+--:::::=*=::::---------------------------------------------
// :::::::::::::::::::::::::::::::::::::::::-==++++++++==+**********########**+=-:::::-:::::----------------------------------------------
// ::::::::::::::::::::::::::::::::::::::::::-=+++++++++=+++*******#########*+==-::::::::.::----------------------------------------------
// ::::::::::::::::::::::::::::::::::::::::::==++****++++++++******######*+===---:-:::::..::----------------------------------------------
// ::::::::::::::::::::::::::::::::::::::::.:==++******+++++++******#####*+=--:::-:::::...::::-------------------------------------:---:--
// ::::::::::::::::::::::::::::::::::::::::.-==++*******++++++*****######*++=--::-:::::...::::-::--:-::---------:--------::---::::::::::::
// :::::::::::::::::::::::::::::::::::::-+..==+++****#****++******########**+=------::::...::::::::-:::::-:--::::-::::::::::::::::::::::::
// ::::::::::::::::::::::::::::::::::::=++..=++++****#********#*############*++=--=--:::.: +-:::::::::::::::::::::::::::::::::::::::::::::
// :::::::::::::::::::::::::::::::::-=++#=..:++++****##*******###############*+=====--::-: -#=::::::::::::::::::::::::::::::::::::::::::::
// :::::::::::::::::::::::::::::::-=+++#%=.:.:+********#******#############*=--====---::-. .*#*-::::::::::::::::::::::::::::::::::::::::::
// :::::::::::::::::::::::::::::=++***#%%=.::::=*#******#*****############+--:-=====---=: ..+%##*-::::::::::::::::::::::::::::::::::::::::
// :::::::::::::::::::::::::--=+**#####%%*..::::-+##***####**############*=---==++==-=+:....=%%%##*-::::::::::::::::::::::::::::::::::::::
// :::::::::::::::::::---=+*#**#####%%%%%%:.::-::-++*####################*+===+++++=+=:..:..=%%%%%%%*-::::::::::::::::::::::::::::::::::::
// :::::::::::::::---=++*#%%%#%%%%%%%%%%%%-:.:--:--==+*%%#################*+****++*+-:..:..:+%%%%%%%%%*+-:::::::::::::::::::::::::::::::::
// ::::---:::--====+*#%%%%%%%%%%%%%%%%%%%%#:::.------=+=*%%%##################*##+--::::::-:+%%%%%%%%%%%%#*-::::::::::::::::::::::::::::::
// ::::--=-===+***##%%%%%%%%%%%%%%%%%%%%%%%=::::-=----+++=*%%%%###%############*=---::--::--*%%%%%%%%%%%%%%%#*=-::::::::::::::::::::::::::
// -====+++***##%%%%%%%%%%%%%%%%%%%%%%%%%%@#-::::-==---=++==*%%%#########%%%#*=-=-:----::-:-#%%%%%%%%%%%%%@@%%%##+=-::::::::::::::::::::::
// +++***##%%%%%%%%@@%@@@%%%%%%%%%%%%%%%%%@%+:-::--===-==+++==-*%%%%%%%%%%#+=-==----=-:----=#@@%%%%%%%%%%%%@@%%%%%#**+=-::::::::::::::::::
// ####%%%%%%%%%%@@@@@@@%%%%%%%%%%%%%%%%%%@%#-:---=-===-=-+++==-:-*%%%#**=---==----==------+%@%%%%%%%%%%%@%@@@@@%%%%##**+=-:::::::::::::::
// #%%%%%%%%%@@@@@@@@@@@%%%%%%%%%%%%%%%%%%@@%+::-=-=-===---=++++*#%%%%%%%#+===:--==---==--=*%@@%@@@%%@@@@@@@@@@@@@@%%%%###*++=-:::::::::::
// %%%%%%@@@@@@@@@@@@@@@%%%%%@%%%%%%%%%%%@@@%#=:-=+=-------+*%%%%%%%%%%%%%%%%#*=-=---==--=+#@@@@%%%%%@@@@@@@@@@@@@@@@@%%%%%##**++=-:::::::
// @@@%%@@@@@@@@@@@@@@@@%%%%@%%%%%%%%%%%%%@@@%*=--=-:.:-*%%%%%%%%%%%%%%%%%####%%%#+-=----=+%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%%%%#***++=-:::
// @@@@@@@@@@@@@@@@@@@@%%%@@@@@%%%%%%%%%@@@@@%*+-:.:=*%%%%%%%%####%%%%%%%%###%%%%%#%*=--==#@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%%%%##***++=
// @@@@@@@@@@@@@@@@@@@@%%@@@@@@@@%%%%%%%@@@@@%*+++*##**##%%%%%#####%%%%%%###%#%#**++++%#**%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%%%###**
// @@@@@@@@@@@@@@@@@@@%%@@@@@@@@@@@%%%@@@@@@@@%%%##***+**##%%%%#####%%%%%%%%#*##**=+=*###%%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%%%
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%@@@@@@@@@%###*++++**#####%%%###%%%%%%%#**#**+===**###@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@#*++==++**######%%**#%%%%%%%#**#**+==+***#%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
    
    let mut a = vec![blank_v.clone()];
    let mut b = a.clone();
    let mut c = a.clone();

    let mut dummy = DMatrix::<f64>::zeros(1, 1);

    let mut z: Vec<DVector<f64>> = vec![];
    while mu < u8::MAX {
        println!("mu: {}", mu);
        gamma_build_next(&w, mu, &mut gamma, order);
        // println!("{}", gamma[mu as usize]);
        for j in 1..order {
            let p_mu_j = p[mu as usize].row(j as usize);
            let tmp = p_mu_j.component_mul(&p_mu_j);
            gamma[mu as usize][(j as usize, 0)] = {
                //shit goes here
                let res = tmp.clone() * w[mu as usize].clone();
                res[(0, 0)]
            };
            // print!("Gamma{} done!", j);
            a[mu as usize][(j as usize, 0)] = {
                //same here
                let tmp2 = tmp.component_mul(&x.transpose());
                let tmp3 = tmp2.component_div(&w[mu as usize].transpose());
                let tmp4 = tmp3.clone() * tmp3.transpose();
                tmp4[(0, 0)].sqrt() / gamma[mu as usize][(j as usize, 0)]
            };
            // print!("a{} done", j);
            drop(tmp);
            b[mu as usize][(j as usize, 0)] = {
                let tmp = p_mu_j.component_mul(&p[mu as usize].row({ j - 1 } as usize));
                let tmp2 = tmp.component_mul(&x.transpose());
                let tmp3 = tmp2.component_div(&w[mu as usize].transpose());
                let tmp4 = tmp3.clone() * tmp3.transpose();
                tmp4[(0, 0)].sqrt() / gamma[mu as usize][({ j - 1 } as usize, 0)]
            };
            // print!("b{} done",mu);
            c[mu as usize][(j as usize, 0)] = {
                let tmp = p_mu_j.component_mul(&y.transpose());
                let tmp2 = tmp.component_div(&w[mu as usize].transpose());
                let tmp3 = tmp2.clone() * tmp2.transpose();
                tmp3[(0, 0)].sqrt()
            };
            // print!("c{} done", j);
            let row = {
                let a_vec = DVector::from_vec(vec![a[mu as usize][(j as usize, 0)]; l]);
                let c1 = x.clone() - a_vec;
                let b_vec = DVector::from_vec(vec![b[mu as usize][(j as usize, 0)]; l]);
                let add = p_mu_j.component_mul(&c1.transpose());
                let subs = p[mu as usize]
                    .row({ j - 1 } as usize)
                    .component_mul(&b_vec.transpose());
                &(add + subs)
            };
            p[mu as usize].set_row({j+1} as usize, row);
            // print!("p{} done", j);
            println!("j: {}", j);
        }
        std::mem::swap(&mut p[mu as usize], &mut dummy); // Crazy shit!!!
        dummy = dummy.remove_row({order-1} as usize);
        std::mem::swap(&mut p[mu as usize], &mut dummy); // TOP 5 UNO-REVERSE ANIME MOMENTS!
        println!("Came throught insanity");
        // let debub_c = c[mu as usize].clone().transpose();
        println!("{}",p[mu as usize]);
        z.push({ c[mu as usize].clone().transpose() * p[mu as usize].clone() }.transpose());
        E.push(
            {
                let tmp = DMatrix::<f64>::from_diagonal(&{ y.clone() - &z[mu as usize] });
                DMatrix::<f64>::from_diagonal(&w[mu as usize])
                    .pseudo_inverse(1e-6)
                    .expect("Should be able to pseudo inverse E")
                    .diagonal()
                    .transpose()
                    * { tmp.clone() * tmp }.diagonal()
            }[(0, 0)],
        );
        f = <usize as TryInto<u8>>::try_into(l).expect("Should fit in value f") - n - m;
        println!("Pushed success");
        if (E[mu as usize]
            < (<u8 as TryInto<f32>>::try_into(f + m).expect("Should be able to convert f + m")
                + <u8 as TryInto<f32>>::try_into(2 * f)
                    .expect("Sould be able to convert 2*f")
                    .sqrt())
            .into())
        {
            println!("HELP");
            return z[mu as usize].as_slice().to_vec();
        };
        let mut tmp = DVector::<f64>::zeros(l);
        for j in 0..order {
            sigma[mu as usize][(j as usize, 0)] =
                (E[mu as usize] / (f as f64 * gamma[mu as usize][(j as usize, 0)])).sqrt();
        }
        println!("New sigma is built");
        m = 0;
        for i in 0..l {
            if (y[(i, 0)] > z[mu as usize][(i, 0)] + 2.0 * z[mu as usize][(i, 0)].sqrt()) {
                tmp[(i, 0)] = (y[(i, 0)] - z[mu as usize][(i, 0)]).powi(2);
            } else {
                tmp[(i, 0)] = z[mu as usize][(i, 0)];
                m = m + 1;
            }
        }
        println!("TMP is built");
        w.push(tmp);
        if mu > 0 {
            let all_eq =
            (0..order).all(|j|
                c[mu as usize][(j as usize, 0)] - sigma[mu as usize][(j as usize, 0)]
                    < c[{ mu - 1 } as usize][(j as usize, 0)]
                    && c[{ mu - 1 } as usize][(j as usize, 0)]
                        < c[mu as usize][(j as usize, 0)] + sigma[mu as usize][(j as usize, 0)]);
            if all_eq{
                let less_than_p = sigma[mu as usize][n as usize]/c[mu as usize][n as usize] < 1.0;
                if less_than_p{
                    if ud{return z[mu as usize].as_slice().to_vec();}
                }
                n=n+1;
            } else {
                ud = true;
                n = n - 1;
            }
        }
        println!("Check completed");
        p.push(blank_p.clone());
        // println!("1");
        mu = mu + 1;
        // println!("2");
        gamma_build_next(&w, mu, &mut gamma, order);
        sigma.push(blank_v.clone());
        // println!("3");
        a.push(blank_v.clone());
        // println!("4");
        b.push(blank_v.clone());
        // println!("5");
        c.push(blank_v.clone());
        // println!("6");
        println!("Pushed success");
    }
    print!("{}", z[0]);
    print!("{}", E[0]);
    print!("{}", p[0]);
    print!("FINNISHED WITH MU OVERFLOW");
    z[mu as usize].as_slice().to_vec()
}
