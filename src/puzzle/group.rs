use crate::puzzle::perm::Permutation;

pub struct GroupData {
    cell_recenter: Vec<Permutation>,
    inv_cell_recenter: Vec<Permutation>,
    face_recenter: Vec<Permutation>,
    inv_face_recenter: Vec<Permutation>,
    face_rot: Vec<Permutation>,
}

impl GroupData {
    // Assume rot is a face rotation, rf recenters an adjacent face, rc recenters an adjacent cell
    pub fn setup(
        rot: &Permutation,
        rf: &Permutation,
        rc: &Permutation,
        faces: &Vec<usize>,
    ) -> Self {
        let inv_cell_recenter = Self::setup_inv_cell_recenter(rot, rf, rc);
        let cell_recenter: Vec<Permutation> =
            inv_cell_recenter.iter().map(|p| p.inverse()).collect();
        let mut num_faces = 0;
        let mut is_face = vec![false; rot.deg];
        for &f in faces {
            num_faces += 1;
            is_face[f] = true;
        }
        let inv_face_recenter = Self::setup_inv_face_recenter(rot, rf, num_faces, &is_face);
        let face_recenter: Vec<Permutation> =
            inv_face_recenter.iter().map(|p| p.inverse()).collect();
        // face_rot still needs to be determined
        Self {
            cell_recenter,
            inv_cell_recenter,
            face_recenter,
            inv_face_recenter,
            face_rot: vec![],
        }
    }

    pub fn setup_inv_cell_recenter(
        rot: &Permutation,
        rf: &Permutation,
        rc: &Permutation,
    ) -> Vec<Permutation> {
        let mut inv_cell_recenter = vec![Permutation::identity(rot.deg); rot.deg];
        let mut perms_temp = vec![Permutation::identity(rot.deg)];
        let mut p_idx = 0;
        let mut generated = 1;
        while generated < rot.deg {
            let p = &perms_temp[p_idx].clone();
            for r in [rot, rf, rc] {
                let p_new = p.product(r);
                if p_new.perm[0] != 0
                    && Self::insert_if_not_generated(&mut inv_cell_recenter, &p_new, p_new.perm[0])
                {
                    generated += 1;
                }
                perms_temp.push(p_new);
            }
            p_idx += 1;
        }
        inv_cell_recenter
    }

    pub fn setup_inv_face_recenter(
        rot: &Permutation,
        rf: &Permutation,
        num_faces: usize,
        is_face: &Vec<bool>,
    ) -> Vec<Permutation> {
        let mut inv_face_recenter = vec![Permutation::identity(rot.deg); rot.deg];
        let mut perms_temp = vec![Permutation::identity(rot.deg)];
        let mut p_idx = 0;
        let mut generated = 1;
        while generated < num_faces {
            let p = &perms_temp[p_idx].clone();
            for r in [rot, rf] {
                let p_new = p.product(r);
                if p_new.perm[0] == 0
                    && p_new.perm[1] != 1
                    && is_face[p_new.perm[1]]
                    && Self::insert_if_not_generated(&mut inv_face_recenter, &p_new, p_new.perm[1])
                {
                    generated += 1;
                }
                perms_temp.push(p_new);
            }
            p_idx += 1;
        }
        inv_face_recenter
    }

    pub fn insert_if_not_generated(v: &mut Vec<Permutation>, p: &Permutation, i: usize) -> bool {
        if v[i].is_identity() {
            v[i] = p.clone();
            return true;
        }
        false
    }
}
