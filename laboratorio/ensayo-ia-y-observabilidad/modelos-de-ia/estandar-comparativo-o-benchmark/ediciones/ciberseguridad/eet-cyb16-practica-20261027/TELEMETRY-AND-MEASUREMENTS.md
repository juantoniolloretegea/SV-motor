# Practical examination telemetry, timing and consumption

29 October 2026 · Revision 1.0 · Open call · SV(25,5).

This document specifies the measurements required for each attempt. It does not publish measurements for a pending examination or certify instrument integration. The [previous multi-format examination method](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/METODO.md) provides reusable distinctions and components; its deadlines, contracts and results are not transferred to this call.

## Identity and attribution

Each measurement must identify the call and revision, attempt, objective Q01–Q25, exact model and configuration, Attacker profile (PA) or Defender profile (PD), document package, authorisation and platform. Civil dates use Coordinated Universal Time (UTC); durations use a monotonic clock. Preserve instrument version, unit, provenance, scope and integrity digest.

Keep the Attacker profile (PA), Defender profile (PD), supervisor and target records distinct. Prior observation by the Defender profile (PD) is an examination condition, not an independent observer or consumption attributable to another participant. Resource use by the shared Manager process is not exclusive candidate consumption. A provider label does not establish the physical location or composition of its infrastructure.

## Measurements and conditions

| Group | Required record | Interpretation boundary |
|---|---|---|
| Delivery | Send start, first received fragment, complete receipt, bytes, status, interruption and cause. | A first fragment is not the first generated token; transport duration does not isolate internal provider processing. |
| Candidate time | Effective time, substantiated pauses, opportunities and expiry under the accepted plan. | Do not infer per-question timings from one combined response. Local cancellation does not prove remote cessation or absence of billing. |
| Practical execution | Preparation, admission, compilation, execution, assessment and recovery, with independent limits. | The proposed ordinary 900-second limit does not define these limits or those of Q24/Q25. |
| Tokens | Input, output, cache and processing or reasoning counts when reported, retaining native names and source responses. | Do not add subsets to their totals again. Different tokenisers are not equivalent linguistic units. |
| Context capacity | Complete package identity and revision, measured input, documented context, output reserve and effective model limits. | Estimates do not establish receipt. Do not silently summarise or truncate documents to fit. |
| Service limits | Quotas, concurrency, per-minute and daily restrictions, check date, balance and authorised budget. | Balance and credit do not authorise spending. A forecast does not guarantee a provider billing cap. |
| Cost | Reserve, estimate, general tariff with currency, date and source, reported amount and reconciled invoice. | These are different quantities. Private credits do not improve rankings; shared-account balance changes are not assigned to an attempt. |
| Local supervisor | Start, samples and closure; central processing unit (CPU), memory and input/output of the supervisor process when observable. | These describe the supervisor, not remote hardware. Missing readings are not zero. |
| Target and hardware | Process and descendant identity, legitimate activity, memory, threads, storage, communications, sensors and relevant cache events. | Each instrument states coverage, exclusions and cadence; listing counters is not proof of complete coverage. |
| Accelerators | Presence, use, observation and shutdown of graphics processing units (GPU), tensor processing units (TPU) and neural processing units (NPU), if used. | A CPU test does not certify an accelerator or completion of its outstanding work. |
| Power and energy | Watts and joules only with an applicable instrument, source, sampling frequency, uncertainty and attribution. | CPU utilisation, time and tokens alone do not establish power or energy. Without an instrument: unavailable. |
| Observation quality | Intended and observed intervals, losses, discards, overflow, closure, custody and recovery. | File integrity alone does not establish temporal continuity or scientific sufficiency. |

Physical processor cache, compilation cache and provider-billed input cache are different objects. For Q19, event counts do not replace the physical reference, negative controls or marker-recovery criterion. The limited instrumental reception does not establish universal applicability to other processors, architectures or accelerators.

## Display and data states

Distinguish **measured**, **reported by the provider**, **estimated**, **unavailable** and **withheld**. Unavailable data require a reason; withheld data exist but are not yet disclosed. Publish zero only when supported by evidence. A missing optional metric is not a candidate failure.

Comparison prioritises security consequences under accepted criteria; cost, tokens and speed cannot compensate for a critical failure. Do not improvise a consequence scale or turn it into an additional polygon position. Without comparable conditions, disclose the limitation instead of inventing a tie-breaker.

## Reuse and pending integration

The Manager already has structures for reported consumption, first fragment, transport duration, economic forecasts, examination timing and local OpenTelemetry observation. Its own-process observer has a five-second target interval and a 32 MiB record limit. This cadence does not establish cache-channel observation or coverage of every host process. Integrity checking and scenario-specific observational sufficiency are separate requirements.

Reception for this call requires joining these components to the twenty-five positions, practical phases and both profiles, preserving authorisation, limits and evidence association. Verify reactions to observer loss, permission withdrawal, expiry, restarts and absent metrics. The previous edition's 1,920 seconds and pause rules are not inherited automatically. The [current timing plan](TIEMPOS-Y-ENTREGAS.md) applies.

Stages 3 and 3 bis remain open: the Q19 physical scenario, complete technical library and the ordinary and accelerated research workflow. Publishing this specification does not certify the complete examination instrumentation.

## Booklets and disclosure

The [public booklet catalogue](BOOKLETS.md) distinguishes available material from material awaiting reception. Publication, documentary admission and delivery to a candidate are separate acts. Transmission to an application programming interface (API), including remote token counting, retains its own restrictions and authorisation requirements.

Solutions, reference programs, reserved variants and answer-revealing records remain private until agreed closure and disclosure review. The same boundary applies to web pages, downloads and structured data. Rights and provider agreements must be reviewed before releasing results; withholding the hardware name is not a substitute.

References: [evidence and publication](EVIDENCIAS-Y-PUBLICACION.md), [platform reception](RECEPCION-DEL-SOPORTE.md), [model validation and authorisation](https://documentos-sv.itvia.online/evaluacion/validacion-autorizacion-modelos/) and [consequences of omission](https://github.com/juantoniolloretegea/SV-motor/blob/main/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/estandar-comparativo-o-benchmark/CONSECUENCIAS-Y-FUENTES.md).

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
