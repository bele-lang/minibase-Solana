use anchor_lang::prelude::*;

declare_id!("94VrDhHHBSJiBSHKEz5iznztfs7mEqeVmQRL3nG37Hjj");

#[program]
pub mod videoteca {
    use super::*;

    pub fn crear_videoteca(context: Context<NuevaVideoteca>, nombre: String) -> Result<()> {
        let owner = context.accounts.owner.key();
        let peliculas: Vec<Pelicula> = Vec::new();

        context.accounts.videoteca.set_inner(Videoteca {
            owner,
            nombre,
            peliculas,
        });

        msg!("Videoteca creada");
        Ok(())
    }

    pub fn agregar_pelicula(
        context: Context<NuevaPelicula>,
        nombre: String,
        duracion: u16,
    ) -> Result<()> {
        require!(
            context.accounts.videoteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let pelicula = Pelicula {
            nombre,
            duracion,
            disponible: true,
        };

        context.accounts.videoteca.peliculas.push(pelicula);
        Ok(())
    }

    pub fn ver_peliculas(context: Context<NuevaPelicula>) -> Result<()> {
        require!(
            context.accounts.videoteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "La lista de peliculas es: {:#?}",
            context.accounts.videoteca.peliculas
        );

        Ok(())
    }

    pub fn eliminar_pelicula(
        context: Context<NuevaPelicula>,
        nombre: String,
    ) -> Result<()> {
        require!(
            context.accounts.videoteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let peliculas = &mut context.accounts.videoteca.peliculas;

        for i in 0..peliculas.len() {
            if peliculas[i].nombre == nombre {
                peliculas.remove(i);
                msg!("Pelicula {} eliminada!", nombre);
                return Ok(());
            }
        }

        Err(Errores::PeliculaNoExiste.into())
    }

    pub fn alternar_estado(
        context: Context<NuevaPelicula>,
        nombre: String,
    ) -> Result<()> {
        require!(
            context.accounts.videoteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let peliculas = &mut context.accounts.videoteca.peliculas;

        for i in 0..peliculas.len() {
            if peliculas[i].nombre == nombre {
                let estado_actual = peliculas[i].disponible;
                peliculas[i].disponible = !estado_actual;

                msg!(
                    "La pelicula {} ahora esta {}",
                    nombre,
                    peliculas[i].disponible
                );

                return Ok(());
            }
        }

        Err(Errores::PeliculaNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error: No eres el administrador.")]
    NoEresElOwner,
    #[msg("Error: La pelicula no existe.")]
    PeliculaNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Videoteca {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    peliculas: Vec<Pelicula>,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct Pelicula {
    #[max_len(60)]
    nombre: String,
    duracion: u16,
    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaVideoteca<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Videoteca::INIT_SPACE + 8,
        seeds = [b"videoteca", owner.key().as_ref()],
        bump
    )]
    pub videoteca: Account<'info, Videoteca>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevaPelicula<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub videoteca: Account<'info, Videoteca>,
}
