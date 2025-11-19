use halo2_proofs::{
    arithmetic::Field,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Circuit, ConstraintSystem, Error, Advice, Instance},
    pasta::pallas,
};

#[derive(Clone)]
struct WeakCreditConfig {
    advice: [Column<Advice>; 3],
    instance: Column<Instance>,
}

struct WeakCreditCircuit {
    age: Value<u32>, // 만 19세 이상
    credit_grade: Value<u32>, // 10등급 이하
    no_delinquencey: Value<u8>, // 연체 없음
}

impl Circuit<pallas::Base> for WeakCreditCircuit {
    type Config = WeakCreditConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn configure(meta: &mut ConstraintSystem<pallas::Base>) -> Self::Config {
        let advice = [meta.advice_column(), meta.advice_column(), meta.advice_column()];
        let instance = meta.instance_column();

        // 1. 만 19세 이상 : birth_year <= 2006 (2025 기준)
        meta.create_gate("age >= 19", |vc| {
            let birth = vc.query_advice(advice[0], Rotation::cur();
            vec![pallas::Base::from(2006u64) - birth]
        });

        // 2. 신용등급 10등급 이하 : grade <= 10
        meta.create_gate("grade <= 10", |vc| {
            let grade = vc.query_advice(advice[1], Rotation::cur());
            vec![grade - pallas::Base::from(10u64)]
        });

        // 3. 연체 없음: delinquency == 0
        meta.create_gate("no delinquency", |vc| {
            let del = vc.query_advice(advice[2], Rotation::cur());
            vec![del]
        });

        WeakCreditConfig { advice, instance }
     }
     fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<pallas::Base>) -> Result<(), Error {
         layouter.assign_region(|| "witness", |mut region| {
             region.assign_advice(|| "birth_year", config.advice[0], 0, || self.age.map(|y| pallas::Base::from(y as u64)))?;
             region.assign_advice(|| "credit_grade", config.advice[1], 0, || self.credit_grade(|g| pallas::Base::from(g as u64)))?;
             region.assign_advice(|| "no_delinquency", config.advice[2], 0, || self.no_delinquency.map(|d| pallas::Base::from(d as u64)))?;
             OK(())
         })
      }
}
