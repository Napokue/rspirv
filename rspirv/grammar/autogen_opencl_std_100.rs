// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static OPENCL_STD_100_INSTRUCTION_TABLE: &'static [ExtendedInstruction<'static>] = &[
    ext_inst!(acos, 0u32, [], [(IdRef, One)]),
    ext_inst!(acosh, 1u32, [], [(IdRef, One)]),
    ext_inst!(acospi, 2u32, [], [(IdRef, One)]),
    ext_inst!(asin, 3u32, [], [(IdRef, One)]),
    ext_inst!(asinh, 4u32, [], [(IdRef, One)]),
    ext_inst!(asinpi, 5u32, [], [(IdRef, One)]),
    ext_inst!(atan, 6u32, [], [(IdRef, One)]),
    ext_inst!(atan2, 7u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(atanh, 8u32, [], [(IdRef, One)]),
    ext_inst!(atanpi, 9u32, [], [(IdRef, One)]),
    ext_inst!(atan2pi, 10u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(cbrt, 11u32, [], [(IdRef, One)]),
    ext_inst!(ceil, 12u32, [], [(IdRef, One)]),
    ext_inst!(copysign, 13u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(cos, 14u32, [], [(IdRef, One)]),
    ext_inst!(cosh, 15u32, [], [(IdRef, One)]),
    ext_inst!(cospi, 16u32, [], [(IdRef, One)]),
    ext_inst!(erfc, 17u32, [], [(IdRef, One)]),
    ext_inst!(erf, 18u32, [], [(IdRef, One)]),
    ext_inst!(exp, 19u32, [], [(IdRef, One)]),
    ext_inst!(exp2, 20u32, [], [(IdRef, One)]),
    ext_inst!(exp10, 21u32, [], [(IdRef, One)]),
    ext_inst!(expm1, 22u32, [], [(IdRef, One)]),
    ext_inst!(fabs, 23u32, [], [(IdRef, One)]),
    ext_inst!(fdim, 24u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(floor, 25u32, [], [(IdRef, One)]),
    ext_inst!(fma, 26u32, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(fmax, 27u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(fmin, 28u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(fmod, 29u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(fract, 30u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(frexp, 31u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(hypot, 32u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(ilogb, 33u32, [], [(IdRef, One)]),
    ext_inst!(ldexp, 34u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(lgamma, 35u32, [], [(IdRef, One)]),
    ext_inst!(lgamma_r, 36u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(log, 37u32, [], [(IdRef, One)]),
    ext_inst!(log2, 38u32, [], [(IdRef, One)]),
    ext_inst!(log10, 39u32, [], [(IdRef, One)]),
    ext_inst!(log1p, 40u32, [], [(IdRef, One)]),
    ext_inst!(logb, 41u32, [], [(IdRef, One)]),
    ext_inst!(mad, 42u32, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(maxmag, 43u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(minmag, 44u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(modf, 45u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(nan, 46u32, [], [(IdRef, One)]),
    ext_inst!(nextafter, 47u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(pow, 48u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(pown, 49u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(powr, 50u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(remainder, 51u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        remquo,
        52u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(rint, 53u32, [], [(IdRef, One)]),
    ext_inst!(rootn, 54u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(round, 55u32, [], [(IdRef, One)]),
    ext_inst!(rsqrt, 56u32, [], [(IdRef, One)]),
    ext_inst!(sin, 57u32, [], [(IdRef, One)]),
    ext_inst!(sincos, 58u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(sinh, 59u32, [], [(IdRef, One)]),
    ext_inst!(sinpi, 60u32, [], [(IdRef, One)]),
    ext_inst!(sqrt, 61u32, [], [(IdRef, One)]),
    ext_inst!(tan, 62u32, [], [(IdRef, One)]),
    ext_inst!(tanh, 63u32, [], [(IdRef, One)]),
    ext_inst!(tanpi, 64u32, [], [(IdRef, One)]),
    ext_inst!(tgamma, 65u32, [], [(IdRef, One)]),
    ext_inst!(trunc, 66u32, [], [(IdRef, One)]),
    ext_inst!(half_cos, 67u32, [], [(IdRef, One)]),
    ext_inst!(half_divide, 68u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(half_exp, 69u32, [], [(IdRef, One)]),
    ext_inst!(half_exp2, 70u32, [], [(IdRef, One)]),
    ext_inst!(half_exp10, 71u32, [], [(IdRef, One)]),
    ext_inst!(half_log, 72u32, [], [(IdRef, One)]),
    ext_inst!(half_log2, 73u32, [], [(IdRef, One)]),
    ext_inst!(half_log10, 74u32, [], [(IdRef, One)]),
    ext_inst!(half_powr, 75u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(half_recip, 76u32, [], [(IdRef, One)]),
    ext_inst!(half_rsqrt, 77u32, [], [(IdRef, One)]),
    ext_inst!(half_sin, 78u32, [], [(IdRef, One)]),
    ext_inst!(half_sqrt, 79u32, [], [(IdRef, One)]),
    ext_inst!(half_tan, 80u32, [], [(IdRef, One)]),
    ext_inst!(native_cos, 81u32, [], [(IdRef, One)]),
    ext_inst!(native_divide, 82u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(native_exp, 83u32, [], [(IdRef, One)]),
    ext_inst!(native_exp2, 84u32, [], [(IdRef, One)]),
    ext_inst!(native_exp10, 85u32, [], [(IdRef, One)]),
    ext_inst!(native_log, 86u32, [], [(IdRef, One)]),
    ext_inst!(native_log2, 87u32, [], [(IdRef, One)]),
    ext_inst!(native_log10, 88u32, [], [(IdRef, One)]),
    ext_inst!(native_powr, 89u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(native_recip, 90u32, [], [(IdRef, One)]),
    ext_inst!(native_rsqrt, 91u32, [], [(IdRef, One)]),
    ext_inst!(native_sin, 92u32, [], [(IdRef, One)]),
    ext_inst!(native_sqrt, 93u32, [], [(IdRef, One)]),
    ext_inst!(native_tan, 94u32, [], [(IdRef, One)]),
    ext_inst!(s_abs, 141u32, [], [(IdRef, One)]),
    ext_inst!(s_abs_diff, 142u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(s_add_sat, 143u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(u_add_sat, 144u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(s_hadd, 145u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(u_hadd, 146u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(s_rhadd, 147u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(u_rhadd, 148u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        s_clamp,
        149u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        u_clamp,
        150u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(clz, 151u32, [], [(IdRef, One)]),
    ext_inst!(ctz, 152u32, [], [(IdRef, One)]),
    ext_inst!(
        s_mad_hi,
        153u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        u_mad_sat,
        154u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        s_mad_sat,
        155u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(s_max, 156u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(u_max, 157u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(s_min, 158u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(u_min, 159u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(s_mul_hi, 160u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(rotate, 161u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(s_sub_sat, 162u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(u_sub_sat, 163u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(u_upsample, 164u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(s_upsample, 165u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(popcount, 166u32, [], [(IdRef, One)]),
    ext_inst!(
        s_mad24,
        167u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        u_mad24,
        168u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(s_mul24, 169u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(u_mul24, 170u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(u_abs, 201u32, [], [(IdRef, One)]),
    ext_inst!(u_abs_diff, 202u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(u_mul_hi, 203u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        u_mad_hi,
        204u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        fclamp,
        95u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(degrees, 96u32, [], [(IdRef, One)]),
    ext_inst!(fmax_common, 97u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(fmin_common, 98u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(mix, 99u32, [], [(IdRef, One), (IdRef, One), (IdRef, One)]),
    ext_inst!(radians, 100u32, [], [(IdRef, One)]),
    ext_inst!(step, 101u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        smoothstep,
        102u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(sign, 103u32, [], [(IdRef, One)]),
    ext_inst!(cross, 104u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(distance, 105u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(length, 106u32, [], [(IdRef, One)]),
    ext_inst!(normalize, 107u32, [], [(IdRef, One)]),
    ext_inst!(fast_distance, 108u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(fast_length, 109u32, [], [(IdRef, One)]),
    ext_inst!(fast_normalize, 110u32, [], [(IdRef, One)]),
    ext_inst!(
        bitselect,
        186u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        select,
        187u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        vloadn,
        171u32,
        [],
        [(IdRef, One), (IdRef, One), (LiteralInteger, One)]
    ),
    ext_inst!(
        vstoren,
        172u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(vload_half, 173u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        vload_halfn,
        174u32,
        [],
        [(IdRef, One), (IdRef, One), (LiteralInteger, One)]
    ),
    ext_inst!(
        vstore_half,
        175u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        vstore_half_r,
        176u32,
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (FPRoundingMode, One)
        ]
    ),
    ext_inst!(
        vstore_halfn,
        177u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        vstore_halfn_r,
        178u32,
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (FPRoundingMode, One)
        ]
    ),
    ext_inst!(
        vloada_halfn,
        179u32,
        [],
        [(IdRef, One), (IdRef, One), (LiteralInteger, One)]
    ),
    ext_inst!(
        vstorea_halfn,
        180u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        vstorea_halfn_r,
        181u32,
        [],
        [
            (IdRef, One),
            (IdRef, One),
            (IdRef, One),
            (FPRoundingMode, One)
        ]
    ),
    ext_inst!(shuffle, 182u32, [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        shuffle2,
        183u32,
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(printf, 184u32, [], [(IdRef, One), (IdRef, ZeroOrMore)]),
    ext_inst!(prefetch, 185u32, [], [(IdRef, One), (IdRef, One)]),
];