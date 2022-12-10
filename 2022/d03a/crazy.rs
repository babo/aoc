#![feature(iter_array_chunks)]

fn main() {
    for func in [three::one, three::two] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} ({dur} ns)");
    }
}

mod three {
    fn unique_items(s: &str) -> u64 {
        s.bytes()
            .map(|b| match b {
                b'a'..=b'z' => 1 + b - b'a',
                b'A'..=b'Z' => 27 + b - b'A',
                _ => unreachable!(),
            })
            .fold(0, |acc, b| acc | (1u64 << b))
    }

    pub fn one(input: &str) -> u32 {
        input
            .lines()
            .map(|bag| bag.split_at(bag.len() / 2))
            .map(|(l, r)| [l, r].map(unique_items))
            .map(|[l, r]| u64::trailing_zeros(l & r))
            .sum()
    }

    pub fn two(input: &str) -> u32 {
        input
            .lines()
            .array_chunks::<3>()
            .map(|bags| bags.map(unique_items))
            .map(|[a, b, c]| a & b & c)
            .map(u64::trailing_zeros)
            .sum()
    }
}

const INPUT: &str = r#"gfWpjRRQffQGCHHJsGqjsj
SclzJZZvmmnPbJtVSqqNBqVCBdSCsd
tlbvZJDZtmtPcJmlPnhMFQWWpMRFTfLDRRTWRp
HjMPgSWjVrjgbHRRSSMRgjRdpdbGdlcdCvQfcCdlwQJfdf
LNDnhtNtLNFFZDtFnhzvdldDflvvDCdlJfldpJ
ZFLFZZmFtFtTNTSPRrVPWWMpRP
qLBSBLRwmgzqCbzCffDlrfCV
TFFFHNWFMFFMpHpGHMTHGNhrldWZCsdZsslZlZfrflDVss
PTMcPGntTThHhTGctnMvSwjjvmmqLBmnjqqgCR
nClJtMwwntqVVPJcgZqq
mjpsDcrcSSFFPZqFBWWgVP
vQcjsvhrvvrmhbmNHMNnlHbNMtCtNM
bgvvhnTQtjrrrhsDDf
pLSMltLzLLSjFrSSjrSJHD
zNWRLBdZPllPQtCvttgCqb
DRlDrrFTNDNlgzsGTBfcnqhhcnJfcrCSqc
MMmmdWtdLmvtldHjMmQfPBqSJWnfCCCqcWSSPJ
vjHMjLmjpLtHptQLmHvwTRgNVVpTzZFZgZRlsVTN
rzpMpDCGFCFFjRFsRPFRNFPv
fWclbHCHtSmfvjnmfsvZ
wTcTlSwwtQtWclBQBLGMLMCLVzVLwJGqLd
MQSjLNjPPLLSBPjfQhSPHjDVCjDtVVpDHwbwVpbD
RcmWzsRrzZrmTszWRqWlmRJscbtHwCbndCtcDVddDpdnVnbt
JTsrGGTqmwTlWmTzJzWmhhPLLGgPFgBffSSPhFFM
qMMRNZMDDNWLPqfzCgDcGncVDCgG
wwBFhwhhBgmcVzhghG
tbJbjjtJvwtdtwjpFtlbvtdTLNSMqNqMMgqNHPlZRTNggL
qmjMHsZmZSbjbZMjSLFFFFwgsgvFswpwww
hRJBhmnhhvFFwhcv
llfWDWzrzBNTRfNBrWzzTmZbGTMjPqMmZPjVbSZGSP
CRRPLwwcclcGVppQ
SHFjDjjHDTfSDNTTHfSHjQVGrpmllQQWltVVVZGp
HFlqzDTfqlzwbgPJLwCP
WRCNLphpLppSCWVHNfLRzVnQMnBnMddPMQDFQgrhPQFM
jTjJqvqjvPVJFJFBJF
qTsZbvGqqZlstsmZVljtwqwSHHNWczHSSRcWNSRHzzNfbW
glgzDzHjSrVHcVgbrjmNsscNGmNWssGNNtst
hHPQLHJpwdLpdHfQQtnZmNMwnZGZWwsFZM
QpdhPJRTJfPphJfhCBlVqVvgvVDBbvVqDbHD
VtHzjZpjVtHrprgGmjHsGHNdSJFQRcLJqCdQcSqJNpcq
bBWfTPwhbfDlMnhffRwQJQNdqJcLFQLSdR
bhBhvfMWTnlDnTBfPSmvmjsjmmGtzHtsHm
pcRPRPWrSDcJGZSStmwZZS
VnLfCfTlfVzfnMMBCqVNZJdtjNtJjhJdGNNbwT
BLvqCCMVsnRQsPQgDcZH
cQbqqQhDGhlQfQlhQrqGsTNgLgCpRgLTPPPLNbpg
wtHVddVFwSHznZwwznCpRBdjppNBNTTdCjRR
ZtWFwWtSmvVnwZDrCMGfQlDDJQmD
PzPZGCZzrZrlhdjdCqfCsqQdRD
cbvZLVVFvbbNSNFHSDnsDQdnfqNQDRngsR
FJHSLSFSScJJbWHFmFVFSZmrrzBmhtBwmzBMPMPzPh
nlpFcLBgcVcLbssGVBGGrlpGPhJJJJJqPBZPDNMQMJJhJQZZ
SSTjHzfHwtZSPVQVQMRQ
TzVHwWfTtzwdVzsbFnGgsbdcGrLc
FppVBRVZDdLmrDGmmfrQ
NtNMPNshJCzznLGJSrqRrRrr
tRssthhPlCWhPzsWtzhzCbVVjwTpVwdZZTpwjbdBbwBc
TTWblHWScvPCCHTWFzSrqqsNNSmdmqrrpz
RLRwjjnjZNprzmmZcq
QQgtQnccQDGjgLDRRcLthQhFBvCbMtMHTWlBFllBbFCMTW
WnBVNvDnVsNvZWdrWDLVDMbsHpTjpHCSSClsbSCCMH
GPFtmztzgPhRFtJTdbTwjppSCjpgSl
hJcfPtQhdtWNVZqNnqNQ
GLcqZPPsnqQcFsmBBrqRvrddNqrC
MtHthJwLllwvjRvvtrvBRS
VHMfDLbpfznszZQG
WBSdPlQPRfBtGQPfBGPBJgzgjwsJzsszJwCrdwCT
ZpppVpMVpnVHMVVbZRJrCgwRzTJrwNJw
MvhmnpLqLmhVmBlftRQBFSlR
hhQlSJqhtCSnqZJnqShSlNDwRzpvdwRlMBMMdcjRjMpMRc
frrGmLmWbfFrsmFHmBzBvBcwdJbvpjzbMM
mmgFrVGLWJLFGsgfhSVtVPqntqnnSStN
SFJTJTSqswwFQbwf
cDtcWPclrtPwVsfssQmN
HDtwWCgWdggdzSGJMSzGMq
JpqJtWRJMhCMJpMQCWtFrjgHdgdlgllwNjlQjldH
fBzPZcZvnBmDnZvZBZDmPvglVVVdgHHSwrNRgVgwNPRH
GbZnZccfvcsZmccsmnnZTRbCCMWFTWJqFCCMJFRT
vrrFqrFTBTmLmNrLMqMTHddJbHpWnhdWdWbHhJGM
wBzfwzcQSzWSSshpdWGp
gwjPPPDQtzQlzQDPqTgLBRmRqZBvqFNR
bWVptFFsbPcZsGLhsZGmLB
qnWrnrHdMCDCNqfWmvRRZSSRLdRGZGRG
nNqqNDfMrMWHDQNHzWfHNDnwzblpzFlbwtFbVVlwVcPJpP
BHJhlHdJQggvddglJBBhglhQzZHPZpFFPDMzFDDRDFZZDFZD
rSTfqnCffMfCVfCLNqbzbjWNDbbWDPFpPFbP
nfnnrSfCTVSwrqSLCGfTGlgQhlvsGMJQJBhhssJhGc
tBjjDjjqfDjLfJlrLgglvmrlmrcc
TwNNTVhwwpgvGSNNSssS
TbwhnvvChhbVRTPPRJBJQQfJttMQQJCQfW
mWSvSQVgmWQsQvspQJlrlLnJLLpCClhhlp
bFHRjZdNjjBZzFzhtnCllCcJLrCBll
HFFNHbdZZLZjfPFjHVQmWDDVsvsmTqVqDf
JJPllQQClqgBCgdHwHbpjVTwHd
tmGZtjGjHZpVbfMT
ShGjNGWmDSNcNRtGmshDRzzCvzQJJRBLrvlrBPJv
cTpqsTWqVVpsNLfvCDFlMFDVFL
JnndJPddQgzHlvMJFDhLCG
BjtntgdRnQgzjdBRQBlpNWrTTlNTSwNpWS
qHmqLVLjmVqsDBLtmjmbtPwCTwwPzGWRgGwGwMwW
ZhcCNCSprRTWTwSnWW
hflhZvvQhppZfcNpvrhpQHjVjLmbVmmVHVCFDvqVFb
nnNrwDnZrspwDNnZsNSDsNbCmpjvMTPQjLMmPmmQPGBTQP
FdVtRdRfctBQPmTtTLQB
qhzWVWJqVHwbhlLSsS
htWmhDhFztnztDhtBmBtghPRSrpfjVwPdfPwpwnRSVrr
cbCHvgJGcTqbqcbqqqcqsMsRVrSCwffdRPPpVpwCRSwfjj
GlgGQqTqbgQzttmBNNFz
NWQNQgdTgjQNddTZfrCQWRDnnnbqnLqnRcjJlqqvDj
FtSSmSmJhpllcclDvpln
JBVVSsSFBVBttShFGSPQfCGNdrMfZZTQTZNNdC
HgHthMhphcbfbMMfHhsGGDCRRVlcVSScsCRz
nWvPFqLqPNdjnNLnjdJnPdWjGlssDPSsllVCRzlTCTGlSDzS
RvddJRJQHwQwpZZb
gdZwgpjZZQtHTdrWrwdpWRnlhNBRlLbFthNhflhBnL
CVzDCPGMVqVmGsGGbJCmCDvMcRcqnBFFFnRBBNRBBNqhnFfF
DsmSGsGPzvMGJvdbgTSTbjbSSdgH
jBGmbNBQGdBNNDJNQRLLVDsHtDRzHHZZcH
wCWPFWPCrPhPrplvprhwpCHHtszttqZslRVHLtzVlJZL
vprMMvMnJCwnnPShNGSTfGSfNmmgdNff
bPtLbvVWWztbLSVVnbszpzQsrcDDBdpRcDrs
llZmgCZqgCFgmdRdJcscBdJsmQ
FZlgfqCFfgZHlqCMCglwCFGWntLLSMRSPGPVttWRtVGL
vtnDsDtrnrSvrMVmbrrJgPCmBm
FpQHzFclLVzWHhwHLQLlHLzPmMBQCJTdTmCTmBTJTTmgQg
pllcVWqlffZqZtZD
TSSZWpsQmZWcTZSvsTTTppNPzrBPrNBrzQNVFrBBNPqP
CgjmCbtGgftMmLtLmffzBzJJJNVVMNzNBqJrFN
gjgjLgtLwgbGjHdhhGdvmlnllnpWnplZvcvwTl
htLrRFRtbbhlGSLRtbJBJsjBmgMMgJgtmBzz
pZQWddQQfpZZffcDQZwddQwDMqDDsPgGJJzzjqzgJMBJgmms
QdcQTdwpGNwfrCRlRVlNLSbb
wrdvpVBVpMGPPjWjGZJJZT
tChCSlNfCCHtvHHWPHPZ
RbRRNvmcqcblfMwwdVBQQqqdpL
qcctqRcqmcHWzHBdDMZhfwthBnwt
JFsSNMSgNSNJJMGJBBdjhFDfhwhBrwnZ
TbgbsSgJMTJllblLCSPlsTCVQmRVVWpQzzqpqzVzHLQzcc
CVcWbjjSSCSSnpjWpCpprhHZlHtHGzHrZrHGclrl
gqZqdddLgmgNqvTGGHvvmrrGHT
FFDgZfZNLMgNfdDqDRnsnjBpbSbnMBBWpQpB
qwpQFwRnqFFfSBSfFt
LJJLGLWWtZlbgWHgGshhSdSVzmhHmfVzzC
lrbrbrNNJgDMLLbblGctvvvDqPcqctTTTcqP
vnblvbfHvlcHMlHlZbSPLTPLwCMBRRPRRFFR
tszzBqtzDsWVPRSmzLVmVL
tsNsDDNgGsqBrgBpgdHQbfhflcHdpZvdbh
cCpLtpGGLsgsppcpmGGHMtjfHRVhvvVVFRfhjV
NWnnnNNndQnQZdCdzzRVMHzvhhHWWWjj
CPJJrnSZpGDJLGTL
cnJzpcnmnQVFbzTlvTHBlb
tWCDPjfsDGfZhddhjjdTvFTgFgvbnFHvdHqT
hjfCjwDDGjPthsfhsnGNrJcQcRmJMLVJrJNMLw
CPPRrSlRccPcwTHwfdwTHdfl
mLQLLjhQhhQLZvpzssHDhdTswzzTJD
gmjbBvQLWmgbQZBCSRnnnSMVCBHnBS
sWrBJbsVqschzhQzHh
gtFmztnSlSfdlmnZSdSwcwGRTjcTcwwTcHccRg
FzFDzMZCdDZtCSrJVBMqWVrqNBqN
TvWlhhfhZJVgtSSl
ddBdGGdFmmBbdzqqPDDGGmdDZSgttHtZppSgzZHSgMhtMgtz
PGqdrbbbdPnrcjjhTRWLLc
trrmJWcrVwVbcPScdcBdGPHH
JTQnfjlJTpQFfMLlNJHHGDPdGsSdDjHGDPPH
ffFfnCTTCfTlplTMvNVzqWvwVzrrhwmWhJbW
hVtDtgcghzJpmmhlwp
srsnrqqsPqsBPvnqRBRMPbnwlplpmCStJwmzJPtJzJfwSw
bbrqjBbvGsjGGBWqMVFFVDNVNjZjgtgFgZ
mnmhBDHhwWCHsTgRsH
dcSlFvccMFMMFFggNsTzzvvzWnVW
llQdllZScFplJPpdcZSqBqjhmtnrwrDGnQGhrq
ZffVNgfTdmPVltsnnGwgQDnB
rMCFLMHpzCMFzHpzbrcHFLzBwsDsDDnlDBJrDDBBSJSnBn
MLMjMzqpCzvwqTmwZdvq
DDNlWPRqgPRPsRFjJQZbchJZbgQJ
zzrLLznpLbHnjcBHvVvHvJcZ
ndmrTzbMMTfzrTfnTLrzdpmsPPPqlqGDNNsPCRDRqRsD
zzdqTNfTfdfhgQhgqMFSjRDtDRWHqtWlwtqDRS
ssBCrcmpVGZvVRDdSDRwtmWdDb
rvGPCZLCVCPVBZFdnfThgNgLJNhf
bslcrssQwDPbQrrcsbsnQrjMLthPMMRhLRhLRgzmgPhRgM
DffvDfHGfNFdpfTdMtghLBThzVmBhBtM
SNvJNJdflDDbcDWJ
HFlHNpWsTlGWbFsGFTGHFLLNzPPhLVPMzVzMNPhhzP
jSvZtmrqqpcrCpPVzw
dddQvqDgDmjdSQQdqZjStpffWGgBRWTGfGsRlWBlHF
THnTbNrdBnLTHHnTnBrWRTndsccZsLZcDqmLDPcDlQDsmmsZ
ptwzzhpvGSVdqQlmszqmqPqc
wGVjSddCBggCHFWN
LFFbdbhhhvwvfTNdRhhRRvMbHDGjcfcGfDjtDHHcHqGjDqqj
WlQnVpWSSWWsPsgDqDzHDLHjJcttGP
rrWsZrgVnWrWSlmSlmSBFFbvTThhBFvvZLBhRw
BgBdcjThvjFcTggrqvVfzlnnPlrqLt
JpwJGPsQwpwSssHpPLlzlnNlzLLNNLVtsN
JPMmWGmWPmHbHpJbWGJmDmwbBTRZMBBdZCRTRjFjhCZCCBTT
BjbcLFRfBRhnbGjCVVvPllpcPtcDmdlPpvPP
WrMQqCNgsqWWsTNCMZMWWsWPvJDJDddvlpDtZDpDDDDwvP
qNMzzSzSQsGLbFCSCnVR
tTRpHJQpQBZcddhhMhvhJN
zswljflgMFbwPqmNmSdvShLNfLhm
qFbsMCVgsqMwRWHCWDDBDWpt
VSTCCWsJvGpHHCNC
GrqzZrrZjDljcDDlfjMqgRPfPvQPpBHNvHvBpvNQ
rljncDcznjMqhlhZDnltrzhTsGWtbVLFTTWGsbdWJdFTmL
mJPDSJJPZPJNrprSNrDmpZGrhFFhBqjGbGGVbFjhhfqBjBRV
cgnTQHdMQdTHdhqfggBhVqVfVS
nQdLLddssSJrmsNvZrPz
jfjffQzZQQMzZZfZZQFgjDWBCRlCBdTTBGGGRpBCgdhdBG
LrstWtNsbHLsprRBdlGpCwlh
HLnntbnscqLvvPNNfMWSSmDMDPjzjDzS
vhcGwWVvglltcfBn
BBSLrzSJLzJNJrLfPfPRsmDRmflD
jMjFZJNMqzrzZzFNFjNQqJzbCpBBvWdpvTCWhpVwdvHVCGbG
HlrnFmRmtRBQPVBTQHHQ
psSLJsLpTTdPdLTv
fCGgTgfSSCtRtFFzql
pfTpStppcDlWfbpDdzQRsQGJhfffQgJHzN
ZFZFZmBFwVwBVmLmLsRLRhHNzRLRNNzJ
FnnjwVPmnqqqjBjrTdblldCTpcPJtbTD
bdZHdWlrjslMMwGG
rDDTRBTqSqmJLBJRBTSJpmMsMMjhwvfMhjjfVGsLshhC
BqQFRPFRQBJgzrcZNHFdZt
wrDdLlDdPWZPTTrwlZpSsPsHVHsSCHnbzMHM
JtNFttNCjFvpppnMpJgSVS
NFFqFcCQCvfrZmGdZdmqrW
GMNNfJnNddJFJWsv
HSDwCmmghLmwmmHDpsvdFpMWpppptSbp
zCzBCgzhwmhzLrPnVrMqZBNfGf
DrHGtbltbCjjjffPrgsmzmcqsgDczdsmgJ
VZLwQLZLLVwLBQZnLVphhLQQqsTNmzJdcNTzzmJNqlNBsszz
wZLhVMplpQVRRlpVGPfjCjMGCrbHGWWb
BHpFrHHbBNTWWTWNhCPwPLNPjCdjLV
zJRRzJvZlcZsSMJdzSDjDtfDCtDtjDjjjj
dcJcszQJJGRJzRllMpGHpFTWmrTmBTbWWB
qnWWqhDhnjmjCMBlNRrfVfRNCB
vvBLBtGHJTHBddrNVJrVSVdr
BZLTHbgvHvTFBgTFFvhmWmmZDPmmZDsnqncs
WBvmjDbSzTMmHHdpNHNF
ttlflZRfGtfWVRltGtflCdHnJrNJHNHnJddNMNCnpF
VVwssWQQfRGZcszBQzDbjSBvSBDP
lSlQqQVqWWVWfqQWVJSTscdmPPwwTTmjjfpjPp
FCbzHbvHvtgrtFCvbvbbwdTwmsrwnTTpmdswmwcc
DtZbHdghztlLMQlWWhVQ
pqzzFSmdFqbQvlpdDGGrGBWPPBVNQnVttZ
cgcjwfBMhHCjjLMCrtcnPcsnsPGVnrVs
JgCChjjjBHhRRLLjjhplzvzpSFJvzzlDbSqm
mZzVQZMhmrffwfQhWhzmrmpBtRcdbnbcdcMpBbDbncdD
jsLTSlTWRBSDpnDn
GLTsGWGFsfmJGZVJZm
BGWshBGnsFWSLWBLlSSLWRJHnrVPrPcNHCNHctnPPJ
QmvQCqqMTZqvgmvTjpZCMgMtrVctPptHtrNVrptbJJbrRP
CzjCZfCwDzShDWdF
HmQlQHmJnpmptmzt
MTqMjMPvTvVvhpdztZnSwzwZqS
CcbLLPTMtCCsjHNHQFLRRFlRNN
GDFwLLLLSrbdPlFBMFsslFHmZH
TnJCgthHpVTfZMQZQmzWnZ
hjvtjtghtqJvVjhTgNhJTvdvdDDRbbccrwPdcGwrHS
MQQMBPzMGQBPBbDQPMhpnRwsGnRhNrFFpRnF
vmgHcmCTTlvvvZvTmqcTfmCRdddFnwdRdnVwFpVfpRnwNw
gvmqJTcHclCQJNzjMLWbLj
DbqqDDbQFqfNtZSLSq
RrdjPdmrpWBdmWRdccfLtNttSDMZBfftLMLf
dCcgmgRrWcgcppjCVVVVFHFnDnbJnb
fZMFfrtVdZSDVwTgjRMLhwTCLj
cNzPBNpclllzHbmTNRhqCRTgjC
nhhWJzhGPlQcGvsvfJtSfZfrtt
PSzrBWQBBGzBlnSnWtDrqHfNfwVwHcLNjHjwcDNmFH
hbRhtRCRpRvsRgVVVcNHNNNCwLwc
ZtRTRvttWWzBPlGZ
tcLnctNsJrWWNDTN
pwPPSjHSHHfzvmSvvvFVVGqGVqGmFqrDWgDr
pPSvfPQMzCQCSbhllLnQDhbtQZ
DmLffDhpVhjjVwvbwNVFbbNSNH
JRPBgMPRHBrMHMHqrBMqWJBSQQNbCvndNrdvCNCFwFrQnv
WcqJcPGMGtWRRBtgZjjspGHTLHGHTppm
ptJtWJpqRwDZZDVWpbDWqlvvflfMjlfCMjdCCdtslv
rLwTBGBzBBQTzmwCCjvdvlLllddsMl
NBwTmrGNgrTrcgPpWgWPDSVVPW
CdglMnrlSSqDPpcsZb
ccwmVJtvVvVtNhBpBFPDVpqbbD
TRGQjJjGTmtrTCgHWLfrcn
JNNhLwWwWQHNPDmmjHpc
zMqZCvVCSMVqMSTVvZVGsBnlslpmsmzlPmsHPsPB
qTVqrgdCCbhfHJQFtg
wNwCBBCZsfQWfmLCGSmmFRGSSF
zjnPHPVqMhhZLTcbpbSncp
lVlhlgzlPZlwtgBddJdfvf
JWRWRRLWJLnjtjnLzGzznflBvfPvPMqMDqdbzblCzC
TTScTVbHmTsVFrmcsgcHFlPMMvlvrDPdlrDDqdldvl
bVpcpchgsFZHbhSmSTsHFFjwtZjnjLttntNjLjNLWtjw
rffjPJzWzrgPpGWHVNqTtmqFTVRH
cswhvlLBvSLsCtbFccmqVFNTbb
wwZSCZSnCLsSDGgDmpGnfmmr
rTfJTNtjfNljlrWSlzRtNlTqsddwGnsnHHwwhssTsnqw
VpbpZZbvPLbZbbBhwqMHhsGMnJdVwV
mgQZJDLBJbbbcbgZClCSfWlrCjRjlDCR
fSpwcVfzsztcSSWNNMbnMRqTvtTv
mJFmGDDDhGhBJHCQddllqTvCllqTRRWNnMbT
FdFDGdDDDhhHdZDjhDmpwSPVZszpwZsVgsPRZs"#;
