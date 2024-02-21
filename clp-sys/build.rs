use std::env;

fn make_builder() -> cc::Build {
    let target = env::var("TARGET").expect("Could not find TARGET in environment.");
    let mut builder = cc::Build::new()
        .cpp(true)
        .warnings(false)
        .extra_warnings(false)
        .define("NDEBUG", None)
        .define("HAVE_STDIO_H", None)
        .define("HAVE_STDLIB_H", None)
        .define("HAVE_STRING_H", None)
        .define("HAVE_INTTYPES_H", None)
        .define("HAVE_STDINT_H", None)
        .define("HAVE_STRINGS_H", None)
        .define("HAVE_SYS_TYPES_H", None)
        .define("HAVE_SYS_STAT_H", None)
        .define("HAVE_UNISTD_H", None)
        .define("HAVE_CMATH", None)
        .define("HAVE_CFLOAT", None)
        // .define("HAVE_DLFCN_H", None)
        .define("HAVE_MEMORY_H", None)
        .to_owned();

    if target.contains("msvc") {
        builder.flag("-EHsc").flag_if_supported("-std:c++11");
    } else {
        builder.flag("-std=c++11").flag("-w");
    }

    builder
}

fn main() {
    const COIN_UTILS_SRC: &str = "vendor/CoinUtils/src";
    const COIN_CLP_SRC: &str = "vendor/Clp/src";

    // Compile CoinUtils
    let mut builder = make_builder();

    builder
        .flag(&format!("-I{}", COIN_UTILS_SRC))
        .file(format!("{}/CoinAlloc.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinBuild.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinDenseFactorization.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinDenseVector.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinError.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinFactorization1.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinFactorization2.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinFactorization3.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinFactorization4.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinFileIO.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinFinite.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinIndexedVector.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinLpIO.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinMessage.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinMessageHandler.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinModel.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinModelUseful2.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinModelUseful.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinMpsIO.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinOslFactorization2.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinOslFactorization3.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinOslFactorization.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPackedMatrix.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPackedVectorBase.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPackedVector.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinParam.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinParamUtils.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPostsolveMatrix.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPrePostsolveMatrix.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveDoubleton.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveDual.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveDupcol.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveEmpty.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveFixed.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveForcing.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveHelperFunctions.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveImpliedFree.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveIsolated.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveMatrix.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveMonitor.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolvePsdebug.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveSingleton.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveSubst.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveTighten.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveTripleton.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveUseless.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinPresolveZeros.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinRational.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinSearchTree.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinShallowPackedVector.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinSimpFactorization.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinSnapshot.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinStructuredModel.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinWarmStartBasis.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinWarmStartDual.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinWarmStartPrimalDual.cpp", COIN_UTILS_SRC))
        .file(format!("{}/CoinWarmStartVector.cpp", COIN_UTILS_SRC))
        .compile("CoinUtils");

    // Compile CoinUtils

    let mut builder = make_builder();

    builder
        .flag(&format!("-I{}", COIN_UTILS_SRC))
        .flag(&format!("-I{}", COIN_CLP_SRC))
        .file(format!("{}/ClpCholeskyBase.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpCholeskyDense.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpCholeskyPardiso.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpCholeskyTaucs.cpp", COIN_CLP_SRC))
        // Need to have AMD or CHOLMOD to compile ClpCholeskyUfl.
        //.file(format!("{}/ClpCholeskyUfl.cpp", COIN_CLP_SRC))
        //.file(format!("{}/ClpCholeskyWssmp.cpp", COIN_CLP_SRC))
        //.file(format!("{}/ClpCholeskyWssmpKKT.cpp", COIN_CLP_SRC))
        .file(format!("{}/Clp_C_Interface.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpConstraint.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpConstraintLinear.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpConstraintQuadratic.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpDualRowDantzig.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpDualRowPivot.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpDualRowSteepest.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpDummyMatrix.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpDynamicExampleMatrix.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpDynamicMatrix.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpEventHandler.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpFactorization.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpGubDynamicMatrix.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpGubMatrix.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpHelperFunctions.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpInterior.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpLinearObjective.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpLsqr.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpMatrixBase.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpMessage.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpModel.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpNetworkBasis.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpNetworkMatrix.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpNode.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpObjective.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpNonLinearCost.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPackedMatrix.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPdcoBase.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPdco.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPEDualRowDantzig.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPEDualRowSteepest.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPEPrimalColumnDantzig.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPEPrimalColumnSteepest.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPESimplex.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPlusMinusOneMatrix.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPredictorCorrector.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPresolve.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPrimalColumnDantzig.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPrimalColumnPivot.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpPrimalColumnSteepest.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpQuadraticObjective.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpSimplex.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpSimplexDual.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpSimplexNonlinear.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpSimplexOther.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpSimplexPrimal.cpp", COIN_CLP_SRC))
        .file(format!("{}/ClpSolve.cpp", COIN_CLP_SRC))
        // .file(format!("{}/ClpSolver.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcBaseFactorization1.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcBaseFactorization2.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcBaseFactorization3.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcBaseFactorization4.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcBaseFactorization5.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcDenseFactorization.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcFactorization1.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcFactorization2.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcFactorization3.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcFactorization4.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcFactorization5.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcHelperFunctions.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcOrderedFactorization1.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcOrderedFactorization2.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcOrderedFactorization3.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcOrderedFactorization4.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcOrderedFactorization5.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcSmallFactorization1.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcSmallFactorization2.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcSmallFactorization3.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcSmallFactorization4.cpp", COIN_CLP_SRC))
        // .file(format!("{}/CoinAbcSmallFactorization5.cpp", COIN_CLP_SRC))
        .file(format!("{}/Idiot.cpp", COIN_CLP_SRC))
        .file(format!("{}/IdiSolve.cpp", COIN_CLP_SRC))
        .compile("Clp");
}
