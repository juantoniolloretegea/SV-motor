"""
Tests del motor SV — suite completa de casos canónicos.
Todos los casos proceden del corpus oficial: Doc1, Doc2, Doc3.
"""
import pytest
from sv_motor.algebra.core import (
    threshold, classify_cell, summarize_cell,
    gate, gate_chain, gate_value, gate_vector,
    gamma_h_labels, gamma_bar_h, kappa3, resolve_policy,
    U, K3_APTO, K3_INDETERMINADO, K3_NO_APTO,
)
from sv_motor.algebra.nlp import (
    i_nlp, run_agent, Observables, H_NLP_SUPPORT_BASE,
)


# ─────────────────────────────────────────────────────────────
# Núcleo algebraico
# ─────────────────────────────────────────────────────────────
class TestThreshold:
    def test_t9(self):  assert threshold(9) == 7
    def test_t36(self): assert threshold(36) == 28
    def test_t27(self): assert threshold(27) == 21


class TestClassifyCell:
    def test_apto_all_zeros(self):
        assert classify_cell([0]*9) == K3_APTO

    def test_no_apto_all_ones(self):
        assert classify_cell([1]*9) == K3_NO_APTO

    def test_indeterminado_mixed(self):
        assert classify_cell([0,0,0,0,0,0,U,U,U]) == K3_INDETERMINADO

    def test_apto_threshold_exact(self):
        # n0=7 ≥ T(9)=7
        assert classify_cell([0,0,0,0,0,0,0,1,U]) == K3_APTO

    def test_no_apto_threshold_exact(self):
        # n1=7 ≥ T(9)=7
        assert classify_cell([1,1,1,1,1,1,1,0,0]) == K3_NO_APTO

    def test_caso_e_doc2(self):
        # Caso E: [1,1,1,0,1,1,1,1,1] n1=8≥7 → NO_APTO
        assert classify_cell([1,1,1,0,1,1,1,1,1]) == K3_NO_APTO


class TestGate:
    def test_all_9_combinations(self):
        table = {
            (K3_APTO,          K3_APTO):          K3_APTO,
            (K3_APTO,          K3_INDETERMINADO): K3_INDETERMINADO,
            (K3_APTO,          K3_NO_APTO):       K3_NO_APTO,
            (K3_INDETERMINADO, K3_APTO):          K3_INDETERMINADO,
            (K3_INDETERMINADO, K3_INDETERMINADO): K3_INDETERMINADO,
            (K3_INDETERMINADO, K3_NO_APTO):       K3_NO_APTO,
            (K3_NO_APTO,       K3_APTO):          K3_NO_APTO,
            (K3_NO_APTO,       K3_INDETERMINADO): K3_NO_APTO,
            (K3_NO_APTO,       K3_NO_APTO):       K3_NO_APTO,
        }
        for (a, b), expected in table.items():
            assert gate(a, b) == expected, f"gate({a},{b}) debería ser {expected}"

    def test_gate_chain(self):
        assert gate_chain([K3_APTO, K3_APTO, K3_APTO]) == K3_APTO
        assert gate_chain([K3_APTO, K3_INDETERMINADO]) == K3_INDETERMINADO
        assert gate_chain([K3_INDETERMINADO, K3_NO_APTO]) == K3_NO_APTO


class TestGateVector:
    def test_all_9_positional_combinations(self):
        expected = {
            (0, 0): 0, (0, 1): 1, (0, U): U,
            (1, 0): 1, (1, 1): 1, (1, U): 1,
            (U, 0): U, (U, 1): 1, (U, U): U,
        }
        for (a, b), exp in expected.items():
            assert gate_value(a, b) == exp

    def test_gate_vector_shape(self):
        v1 = [0, U, 1, 0, 0, 0, U, 0, 0]
        v2 = [0, 0, 0, 1, 0, 0, 0, 0, U]
        res = gate_vector(v1, v2)
        assert len(res) == 9
        assert res[1] == U   # gate(U,0)=U
        assert res[2] == 1   # gate(1,0)=1
        assert res[3] == 1   # gate(0,1)=1
        assert res[8] == U   # gate(0,U)=U


# ─────────────────────────────────────────────────────────────
# Agente NLP — casos canónicos del corpus
# ─────────────────────────────────────────────────────────────
def _obs(**kw):
    defaults = dict(
        theta="coherente", pi="sin-pregunta", kappa="coherente",
        eta="completa", gamma="alineada", alpha="apropiada",
        mu="sin-ambiguedad", chi="sin-solicitud", psi="en-curso",
    )
    defaults.update(kw)
    return Observables(**defaults)


class TestINLP:
    def test_ejemplo_a(self):
        obs = _obs(pi="indeterminada", mu="indeterminada")
        v = i_nlp(obs)
        assert v == [0, U, 0, 0, 0, 0, U, 0, 0]

    def test_ejemplo_b(self):
        obs = _obs(pi="resuelta", mu="cerrada", psi="cerrado")
        v = i_nlp(obs)
        assert v == [0, 0, 0, 0, 0, 0, 0, 0, 0]

    def test_ejemplo_c(self):
        obs = _obs(gamma="indeterminada", chi="indeterminada", psi="indeterminado")
        v = i_nlp(obs)
        assert v == [0, 0, 0, 0, U, 0, 0, U, U]

    def test_caso_e_contradiccion(self):
        obs = Observables(
            theta="desvio", pi="bloqueada", kappa="contradictoria",
            eta="completa", gamma="bloqueada", alpha="inapropiada",
            mu="incompatible", chi="denegada", psi="bloqueado",
        )
        v = i_nlp(obs)
        assert v == [1, 1, 1, 0, 1, 1, 1, 1, 1]

    def test_normalizacion_acento(self):
        obs = _obs(theta="desvío", mu="sin-ambigüedad")
        v = i_nlp(obs)
        assert v[0] == 1   # desvio
        assert v[6] == 0   # sin-ambiguedad


class TestRunAgent:
    def test_ejemplo_a_indet(self):
        obs = _obs(pi="indeterminada", mu="indeterminada")
        r = run_agent(obs)
        assert r["k3"] == K3_INDETERMINADO
        assert r["gobernable"] is True
        assert r["politica"] == "CONTINUAR_EN_W(T,k)"

    def test_ejemplo_b_apto(self):
        obs = _obs(pi="resuelta", mu="cerrada", psi="cerrado")
        r = run_agent(obs)
        assert r["k3"] == K3_APTO
        assert r["politica"] == "CERRAR_FRAME"

    def test_caso_contradiccion_no_apto(self):
        obs = Observables(
            theta="desvio", pi="bloqueada", kappa="contradictoria",
            eta="completa", gamma="bloqueada", alpha="inapropiada",
            mu="incompatible", chi="denegada", psi="bloqueado",
        )
        r = run_agent(obs)
        assert r["k3"] == K3_NO_APTO
        assert r["politica"] == "PROPONER_FORK"

    def test_irreducible_via_override(self):
        obs = _obs(eta="indeterminada")
        # Forzar P4 irreducible vacciando su soporte
        r = run_agent(obs, support_override={4: set()})
        assert r["k3"] == K3_NO_APTO
        assert 4 in r["U_irr"]

    def test_false_closure_blocked(self):
        """El agente no clausura falsamente una U que el clasificador de umbral cerraría."""
        # [0,U,0,0,0,0,U,0,0] → n0=7≥T=7 → legacy=APTO, pero agente=INDET
        obs = _obs(pi="indeterminada", mu="indeterminada")
        r = run_agent(obs)
        # Legacy K3 (solo umbral) diría APTO; el agente dice INDETERMINADO
        c_frame = r["C_frame9"]
        n0 = sum(v == 0 for v in c_frame)
        t  = threshold(len(c_frame))
        assert n0 >= t                           # legacy diría APTO
        assert r["k3"] == K3_INDETERMINADO       # agente preserva U honesta

    def test_p4_resoluble_preserva_u(self):
        """P4 resoluble: C_gob=U, A_NLP=U, κ₃=INDETERMINADO."""
        obs = _obs(eta="indeterminada")
        r = run_agent(obs)
        assert r["k3"] == K3_INDETERMINADO
        assert r["U_irr"] == []   # resoluble no es irreducible


class TestGammaH:
    def test_fronteriza(self):
        labels = gamma_h_labels([U], {1: {0, 1}})
        assert labels[1] == "fronteriza"

    def test_resoluble(self):
        labels = gamma_h_labels([U], {1: {0}})
        assert labels[1] == "resoluble"

    def test_irreducible(self):
        labels = gamma_h_labels([U], {1: set()})
        assert labels[1] == "irreducible"

    def test_gamma_bar_h_irreducible_produce_1(self):
        sup = {1: {0, 1}, 4: set()}
        v = [0, 0, 0, U, 0, 0, 0, 0, 0]
        cgob = gamma_bar_h(v, sup)
        assert cgob[3] == 1   # P4 irreducible

    def test_gamma_bar_h_resoluble_produce_u(self):
        sup = {4: {0}}
        v = [0, 0, 0, U, 0, 0, 0, 0, 0]
        cgob = gamma_bar_h(v, sup)
        assert cgob[3] == U   # resoluble → U (preserva honestidad)
