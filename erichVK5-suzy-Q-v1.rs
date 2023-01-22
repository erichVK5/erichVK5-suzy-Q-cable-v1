ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
		}
	}
	ha:obj_direct.2 {
		uuid=MdqT+zstH2BY0fhajaEAAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.34 {
				uuid=MdqT+zstH2BY0fhajaEAAAKk;
				x=180000; y=164000;
				li:objects {
					ha:polygon.1 {
						li:outline {
							ha:line { x1=32000; y1=-64000; x2=32000; y2=4000; }
							ha:line { x1=32000; y1=4000; x2=0; y2=4000; }
							ha:line { x1=0; y1=4000; x2=0; y2=-64000; }
							ha:line { x1=0; y1=-64000; x2=32000; y2=-64000; }
						}
						stroke=sym-decor;
					}
					ha:group.2 {
						uuid=MdqT+zstH2BY0fhajaEAAAKl; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAB;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Tx1plus
							pinnum=A2
							role=terminal
						}
					}
					ha:group.3 {
						uuid=MdqT+zstH2BY0fhajaEAAAKm; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAC;
						x=0; y=-4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Tx1minus
							pinnum=A3
							role=terminal
						}
					}
					ha:group.4 {
						uuid=MdqT+zstH2BY0fhajaEAAAKn; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAD;
						x=0; y=-8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=CC1A
							pinnum=A5
							role=terminal
						}
					}
					ha:group.5 {
						uuid=MdqT+zstH2BY0fhajaEAAAKo; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAE;
						x=0; y=-12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=DplusA
							pinnum=A6
							role=terminal
						}
					}
					ha:group.6 {
						uuid=MdqT+zstH2BY0fhajaEAAAKp; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAF;
						x=0; y=-16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=DminusA
							pinnum=A7
							role=terminal
						}
					}
					ha:group.7 {
						uuid=MdqT+zstH2BY0fhajaEAAAKq; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAG;
						x=0; y=-20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SBU1A
							pinnum=A8
							role=terminal
						}
					}
					ha:group.8 {
						uuid=MdqT+zstH2BY0fhajaEAAAKr; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAH;
						x=0; y=-24000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Rx2minus
							pinnum=A10
							role=terminal
						}
					}
					ha:group.9 {
						uuid=MdqT+zstH2BY0fhajaEAAAKs; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAI;
						x=0; y=-28000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Rx2plus
							pinnum=A11
							role=terminal
						}
					}
					ha:group.10 {
						uuid=MdqT+zstH2BY0fhajaEAAAKt; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAJ;
						x=0; y=-32000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Tx2plus
							pinnum=B2
							role=terminal
						}
					}
					ha:group.11 {
						uuid=MdqT+zstH2BY0fhajaEAAAKu; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAK;
						x=0; y=-36000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Tx2minus
							pinnum=B3
							role=terminal
						}
					}
					ha:group.12 {
						uuid=MdqT+zstH2BY0fhajaEAAAKv; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAL;
						x=0; y=-40000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=CC2B
							pinnum=B5
							role=terminal
						}
					}
					ha:group.13 {
						uuid=MdqT+zstH2BY0fhajaEAAAKw; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAM;
						x=0; y=-44000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=DplusB
							pinnum=B6
							role=terminal
						}
					}
					ha:group.14 {
						uuid=MdqT+zstH2BY0fhajaEAAAKx; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAN;
						x=0; y=-48000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=DminusB
							pinnum=B7
							role=terminal
						}
					}
					ha:group.15 {
						uuid=MdqT+zstH2BY0fhajaEAAAKy; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAO;
						x=0; y=-52000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SBU2B
							pinnum=B8
							role=terminal
						}
					}
					ha:group.16 {
						uuid=MdqT+zstH2BY0fhajaEAAAKz; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAP;
						x=0; y=-56000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Rx1minusB
							pinnum=B10
							role=terminal
						}
					}
					ha:group.17 {
						uuid=MdqT+zstH2BY0fhajaEAAAK0; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAQ;
						x=0; y=-60000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Rx1plusB
							pinnum=B11
							role=terminal
						}
					}
					ha:group.18 {
						uuid=MdqT+zstH2BY0fhajaEAAAK1; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAR;
						x=28000; y=4000; rot=90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VbusA4
							pinnum=A4
							role=terminal
						}
					}
					ha:group.19 {
						uuid=MdqT+zstH2BY0fhajaEAAAK2; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAS;
						x=24000; y=4000; rot=90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VbusA9
							pinnum=A9
							role=terminal
						}
					}
					ha:group.20 {
						uuid=MdqT+zstH2BY0fhajaEAAAK3; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAT;
						x=20000; y=4000; rot=90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VbusB4
							pinnum=B4
							role=terminal
						}
					}
					ha:group.21 {
						uuid=MdqT+zstH2BY0fhajaEAAAK4; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAU;
						x=16000; y=4000; rot=90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VbusB9
							pinnum=B9
							role=terminal
						}
					}
					ha:group.22 {
						uuid=MdqT+zstH2BY0fhajaEAAAK5; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAV;
						x=28000; y=-64000; rot=-90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=gndA1
							pinnum=A1
							role=terminal
						}
					}
					ha:group.23 {
						uuid=MdqT+zstH2BY0fhajaEAAAK6; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAX;
						x=24000; y=-64000; rot=-90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=gndA12
							pinnum=A12
							role=terminal
						}
					}
					ha:group.24 {
						uuid=MdqT+zstH2BY0fhajaEAAAK7; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAY;
						x=20000; y=-64000; rot=-90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=gndB1
							pinnum=B1
							role=terminal
						}
					}
					ha:group.25 {
						uuid=MdqT+zstH2BY0fhajaEAAAK8; src_uuid=1/wP+GjcBVzyaNDBLpAAAAAZ;
						x=16000; y=-64000; rot=-90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=gndB12
							pinnum=B12
							role=terminal
						}
					}
					ha:text.26 { x1=0; y1=-68000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=USBC
					role=symbol
				}
			}
			ha:group.35 {
				uuid=MdqT+zstH2BY0fhajaEAAAK9;
				x=56000; y=0;
				li:objects {
					ha:line.1 { x1=140000; y1=176000; x2=152000; y2=176000; stroke=wire; }
					ha:line.2 { x1=152000; y1=176000; x2=152000; y2=172000; stroke=wire; }
					ha:line.3 { x1=148000; y1=172000; x2=148000; y2=176000; stroke=wire; }
					ha:line.4 { x1=148000; y1=176000; x2=148000; y2=176000; stroke=junction; }
					ha:line.5 { x1=144000; y1=176000; x2=144000; y2=172000; stroke=wire; }
					ha:line.6 { x1=144000; y1=176000; x2=144000; y2=176000; stroke=junction; }
					ha:line.8 { x1=140000; y1=172000; x2=140000; y2=180000; stroke=wire; }
					ha:line.9 { x1=140000; y1=176000; x2=140000; y2=176000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.40 {
				uuid=MdqT+zstH2BY0fhajaEAAAK+;
				x=56000; y=0;
				li:objects {
					ha:line.1 { x1=140000; y1=88000; x2=140000; y2=96000; stroke=wire; }
					ha:line.2 { x1=140000; y1=92000; x2=152000; y2=92000; stroke=wire; }
					ha:line.3 { x1=140000; y1=92000; x2=140000; y2=92000; stroke=junction; }
					ha:line.4 { x1=152000; y1=92000; x2=152000; y2=96000; stroke=wire; }
					ha:line.5 { x1=148000; y1=92000; x2=148000; y2=96000; stroke=wire; }
					ha:line.6 { x1=148000; y1=92000; x2=148000; y2=92000; stroke=junction; }
					ha:line.7 { x1=144000; y1=92000; x2=144000; y2=96000; stroke=wire; }
					ha:line.8 { x1=144000; y1=92000; x2=144000; y2=92000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.45 {
				uuid=MdqT+zstH2BY0fhajaEAAALD; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=196000; y=88000;
				li:objects {
					ha:group.1 {
						uuid=MdqT+zstH2BY0fhajaEAAALE; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.47 {
				uuid=MdqT+zstH2BY0fhajaEAAALJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=196000; y=180000;
				li:objects {
					ha:group.1 {
						uuid=MdqT+zstH2BY0fhajaEAAALK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.49 {
				uuid=MdqT+zstH2BY0fhajaEAAALR; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=144000; y=156000;
				li:objects {
					ha:group.1 {
						uuid=MdqT+zstH2BY0fhajaEAAALS; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=MdqT+zstH2BY0fhajaEAAALT; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=10000; y1=2000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=4000; y1=2000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R1
					role=symbol
					value=22k
				}
			}
			ha:group.50 {
				uuid=MdqT+zstH2BY0fhajaEAAALU; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=144000; y=124000;
				li:objects {
					ha:group.1 {
						uuid=MdqT+zstH2BY0fhajaEAAALV; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=MdqT+zstH2BY0fhajaEAAALW; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=10000; y1=2000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=4000; y1=2000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R2
					role=symbol
					value=56k
				}
			}
			ha:group.51 {
				uuid=MdqT+zstH2BY0fhajaEAAALZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=140000; y=160000;
				li:objects {
					ha:group.1 {
						uuid=MdqT+zstH2BY0fhajaEAAALa; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.52 {
				uuid=MdqT+zstH2BY0fhajaEAAALb; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=140000; y=128000;
				li:objects {
					ha:group.1 {
						uuid=MdqT+zstH2BY0fhajaEAAALc; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.53 {
				uuid=MdqT+zstH2BY0fhajaEAAALd;
				x=56000; y=0;
				li:objects {
					ha:line.1 { x1=84000; y1=160000; x2=84000; y2=156000; stroke=wire; }
					ha:line.2 { x1=84000; y1=156000; x2=88000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.56 {
				uuid=MdqT+zstH2BY0fhajaEAAALe;
				x=56000; y=0;
				li:objects {
					ha:line.1 { x1=84000; y1=128000; x2=84000; y2=124000; stroke=wire; }
					ha:line.2 { x1=84000; y1=124000; x2=88000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.59 {
				uuid=MdqT+zstH2BY0fhajaEAAALf;
				x=56000; y=0;
				li:objects {
					ha:line.1 { x1=120000; y1=124000; x2=108000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.62 {
				uuid=MdqT+zstH2BY0fhajaEAAALg;
				x=56000; y=0;
				li:objects {
					ha:line.1 { x1=120000; y1=156000; x2=108000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.65 {
				li:conn {
					/2/35/2
					/2/34/18/1
				}
			}
			ha:connection.66 {
				li:conn {
					/2/35/3
					/2/34/19/1
				}
			}
			ha:connection.67 {
				li:conn {
					/2/35/5
					/2/34/20/1
				}
			}
			ha:connection.68 {
				li:conn {
					/2/35/8
					/2/34/21/1
				}
			}
			ha:connection.69 {
				li:conn {
					/2/40/1
					/2/34/25/1
				}
			}
			ha:connection.70 {
				li:conn {
					/2/40/4
					/2/34/22/1
				}
			}
			ha:connection.71 {
				li:conn {
					/2/40/5
					/2/34/23/1
				}
			}
			ha:connection.72 {
				li:conn {
					/2/40/7
					/2/34/24/1
				}
			}
			ha:connection.73 {
				li:conn {
					/2/45/1/1
					/2/40/1
				}
			}
			ha:connection.74 {
				li:conn {
					/2/47/1/1
					/2/35/8
				}
			}
			ha:connection.77 {
				li:conn {
					/2/56/1
					/2/52/1/1
				}
			}
			ha:connection.78 {
				li:conn {
					/2/56/2
					/2/50/2/1
				}
			}
			ha:connection.79 {
				li:conn {
					/2/59/1
					/2/34/12/1
				}
			}
			ha:connection.80 {
				li:conn {
					/2/59/1
					/2/50/1/1
				}
			}
			ha:connection.81 {
				li:conn {
					/2/62/1
					/2/34/4/1
				}
			}
			ha:group.85 {
				uuid=MdqT+zstH2BY0fhajaEAAALi;
				li:objects {
					ha:line.4 { x1=108000; y1=112000; x2=176000; y2=112000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.86 {
				li:conn {
					/2/34/15/1
					/2/85/4
				}
			}
			ha:group.87 {
				uuid=MdqT+zstH2BY0fhajaEAAALj;
				li:objects {
					ha:line.1 { x1=176000; y1=148000; x2=108000; y2=148000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.88 {
				li:conn {
					/2/34/6/1
					/2/87/1
				}
			}
			ha:connection.92 {
				li:conn {
					/2/34/7/1
					/2/275/2
				}
			}
			ha:group.94 {
				uuid=MdqT+zstH2BY0fhajaEAAAML; src_uuid=MdqT+zstH2BY0fhajaEAAAMF;
				x=108000; y=168000; rot=90.000000;
				li:objects {
					ha:text.1 { x1=4000; y1=6000; rot=270.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=MdqT+zstH2BY0fhajaEAAAMM; src_uuid=MdqT+zstH2BY0fhajaEAAAMG;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=MdqT+zstH2BY0fhajaEAAAMN; src_uuid=MdqT+zstH2BY0fhajaEAAAMH;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=JMP1
					role=symbol
				}
			}
			ha:group.95 {
				uuid=MdqT+zstH2BY0fhajaEAAAMQ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=116000; y=168000;
				li:objects {
					ha:group.1 {
						uuid=MdqT+zstH2BY0fhajaEAAAMR; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.103 {
				uuid=MdqT+zstH2BY0fhajaEAAAMW;
				x=4000; y=8000;
				li:objects {
					ha:line.1 { x1=104000; y1=156000; x2=112000; y2=156000; stroke=wire; }
					ha:line.2 { x1=112000; y1=156000; x2=112000; y2=160000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.147 {
				uuid=MdqT+zstH2BY0fhajaEAAAN4; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=84000; y=100000;
				li:objects {
					ha:group.1 {
						uuid=MdqT+zstH2BY0fhajaEAAAN5; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.154 {
				uuid=MdqT+zstH2BY0fhajaEAAAOb; src_uuid=MdqT+zstH2BY0fhajaEAAAMF;
				x=108000; y=132000; rot=90.000000;
				li:objects {
					ha:text.1 { x1=4000; y1=6000; rot=270.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=MdqT+zstH2BY0fhajaEAAAOc; src_uuid=MdqT+zstH2BY0fhajaEAAAMG;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=MdqT+zstH2BY0fhajaEAAAOd; src_uuid=MdqT+zstH2BY0fhajaEAAAMH;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=JMP2
					role=symbol
				}
			}
			ha:group.155 {
				uuid=MdqT+zstH2BY0fhajaEAAAOe; src_uuid=MdqT+zstH2BY0fhajaEAAAMX;
				x=4000; y=8000;
				li:objects {
					ha:line.1 { x1=104000; y1=120000; x2=112000; y2=120000; stroke=wire; }
					ha:line.2 { x1=112000; y1=120000; x2=112000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.157 {
				uuid=MdqT+zstH2BY0fhajaEAAAOf; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=116000; y=132000;
				li:objects {
					ha:group.1 {
						uuid=MdqT+zstH2BY0fhajaEAAAOg; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.171 {
				uuid=4TDDarqTILvE+aYPfIoAAABk; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=84000; y=136000;
				li:objects {
					ha:group.1 {
						uuid=4TDDarqTILvE+aYPfIoAAABl; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.173 {
				uuid=4TDDarqTILvE+aYPfIoAAABm;
				x=24000; y=4000;
				li:objects {
					ha:line.1 { x1=64000; y1=124000; x2=80000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.176 {
				uuid=4TDDarqTILvE+aYPfIoAAABn;
				x=28000; y=0;
				li:objects {
					ha:line.1 { x1=56000; y1=136000; x2=56000; y2=140000; stroke=wire; }
					ha:line.3 { x1=56000; y1=140000; x2=64000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.181 {
				uuid=4TDDarqTILvE+aYPfIoAAABo;
				x=24000; y=4000;
				li:objects {
					ha:line.1 { x1=60000; y1=96000; x2=60000; y2=100000; stroke=wire; }
					ha:line.3 { x1=60000; y1=100000; x2=68000; y2=100000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.193 {
				li:conn {
					/2/155/1
					/2/154/2/1
				}
			}
			ha:connection.208 {
				li:conn {
					/2/157/1/1
					/2/155/2
				}
			}
			ha:connection.210 {
				li:conn {
					/2/173/1
					/2/154/3/1
				}
			}
			ha:connection.214 {
				li:conn {
					/2/181/1
					/2/147/1/1
				}
			}
			ha:connection.221 {
				li:conn {
					/2/103/1
					/2/94/2/1
				}
			}
			ha:connection.222 {
				li:conn {
					/2/103/2
					/2/95/1/1
				}
			}
			ha:group.223 {
				uuid=4TDDarqTILvE+aYPfIoAAABp;
				x=0; y=-4000;
				li:objects {
					ha:line.1 { x1=88000; y1=168000; x2=104000; y2=168000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.224 {
				li:conn {
					/2/223/1
					/2/94/3/1
				}
			}
			ha:connection.227 {
				li:conn {
					/2/49/1/1
					/2/62/1
				}
			}
			ha:connection.228 {
				li:conn {
					/2/53/1
					/2/51/1/1
				}
			}
			ha:connection.229 {
				li:conn {
					/2/53/2
					/2/49/2/1
				}
			}
			ha:connection.230 {
				li:conn {
					/2/176/1
					/2/171/1/1
				}
			}
			ha:group.262 {
				uuid=4TDDarqTILvE+aYPfIoAAAD4; src_uuid=4TDDarqTILvE+aYPfIoAAADq;
				x=88000; y=128000;
				li:objects {
					ha:polygon.1 {
						li:outline {
							ha:line { x1=-8000; y1=-20000; x2=-8000; y2=-4000; }
							ha:line { x1=-8000; y1=-4000; x2=16000; y2=-4000; }
							ha:line { x1=16000; y1=-4000; x2=16000; y2=-20000; }
							ha:line { x1=16000; y1=-20000; x2=-8000; y2=-20000; }
						}
						stroke=sym-decor;
					}
					ha:group.2 {
						uuid=4TDDarqTILvE+aYPfIoAAAD5; src_uuid=xGiwPkN03typ3E430t0AAAAB;
						x=16000; y=-8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Dplus
							pinnum=3
							role=terminal
						}
					}
					ha:group.3 {
						uuid=4TDDarqTILvE+aYPfIoAAAD6; src_uuid=xGiwPkN03typ3E430t0AAAAC;
						x=16000; y=-16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Dminus
							pinnum=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=4TDDarqTILvE+aYPfIoAAAD7; src_uuid=xGiwPkN03typ3E430t0AAAAD;
						x=0; y=-4000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=5V
							pinnum=1
							role=terminal
						}
					}
					ha:group.5 {
						uuid=4TDDarqTILvE+aYPfIoAAAD8; src_uuid=xGiwPkN03typ3E430t0AAAAE;
						x=-4000; y=-20000; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=4TDDarqTILvE+aYPfIoAAAD9; src_uuid=xGiwPkN03typ3E430t0AAAAF;
						x=0; y=-20000; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SHIELD1
							pinnum=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=4TDDarqTILvE+aYPfIoAAAD+; src_uuid=xGiwPkN03typ3E430t0AAAAG;
						x=4000; y=-20000; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SHIELD2
							pinnum=6
							role=terminal
						}
					}
					ha:text.8 { x1=-14000; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-symbol-generator=boxsym-rnd
					device=USBB
					name=ttyUSB0
					role=symbol
				}
			}
			ha:group.263 {
				uuid=4TDDarqTILvE+aYPfIoAAAD/; src_uuid=4TDDarqTILvE+aYPfIoAAADq;
				x=88000; y=164000;
				li:objects {
					ha:polygon.1 {
						li:outline {
							ha:line { x1=-8000; y1=-20000; x2=-8000; y2=-4000; }
							ha:line { x1=-8000; y1=-4000; x2=16000; y2=-4000; }
							ha:line { x1=16000; y1=-4000; x2=16000; y2=-20000; }
							ha:line { x1=16000; y1=-20000; x2=-8000; y2=-20000; }
						}
						stroke=sym-decor;
					}
					ha:group.2 {
						uuid=4TDDarqTILvE+aYPfIoAAAEA; src_uuid=xGiwPkN03typ3E430t0AAAAB;
						x=16000; y=-8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Dplus
							pinnum=3
							role=terminal
						}
					}
					ha:group.3 {
						uuid=4TDDarqTILvE+aYPfIoAAAEB; src_uuid=xGiwPkN03typ3E430t0AAAAC;
						x=16000; y=-16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Dminus
							pinnum=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=4TDDarqTILvE+aYPfIoAAAEC; src_uuid=xGiwPkN03typ3E430t0AAAAD;
						x=0; y=-4000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=5V
							pinnum=1
							role=terminal
						}
					}
					ha:group.5 {
						uuid=4TDDarqTILvE+aYPfIoAAAED; src_uuid=xGiwPkN03typ3E430t0AAAAE;
						x=-4000; y=-20000; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=4TDDarqTILvE+aYPfIoAAAEE; src_uuid=xGiwPkN03typ3E430t0AAAAF;
						x=0; y=-20000; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SHIELD1
							pinnum=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=4TDDarqTILvE+aYPfIoAAAEF; src_uuid=xGiwPkN03typ3E430t0AAAAG;
						x=4000; y=-20000; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SHIELD2
							pinnum=6
							role=terminal
						}
					}
					ha:text.8 { x1=-14000; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-symbol-generator=boxsym-rnd
					device=USBB
					name=ttyUSB1
					role=symbol
				}
			}
			ha:connection.265 {
				li:conn {
					/2/263/6/1
					/2/176/3
				}
			}
			ha:connection.266 {
				li:conn {
					/2/263/7/1
					/2/176/3
				}
			}
			ha:connection.267 {
				li:conn {
					/2/263/3/1
					/2/87/1
				}
			}
			ha:connection.268 {
				li:conn {
					/2/263/4/1
					/2/223/1
				}
			}
			ha:connection.269 {
				li:conn {
					/2/263/5/1
					/2/176/1
					/2/176/3
				}
			}
			ha:connection.270 {
				li:conn {
					/2/262/4/1
					/2/173/1
				}
			}
			ha:connection.271 {
				li:conn {
					/2/262/5/1
					/2/181/1
					/2/181/3
				}
			}
			ha:connection.272 {
				li:conn {
					/2/262/6/1
					/2/181/3
				}
			}
			ha:connection.273 {
				li:conn {
					/2/262/7/1
					/2/181/3
				}
			}
			ha:connection.274 {
				li:conn {
					/2/85/4
					/2/262/3/1
				}
			}
			ha:group.275 {
				uuid=4TDDarqTILvE+aYPfIoAAAEG;
				li:objects {
					ha:line.1 { x1=108000; y1=120000; x2=132000; y2=120000; stroke=wire; }
					ha:line.2 { x1=132000; y1=144000; x2=176000; y2=144000; stroke=wire; }
					ha:line.3 { x1=132000; y1=120000; x2=132000; y2=144000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.276 {
				li:conn {
					/2/275/1
					/2/262/2/1
				}
			}
			ha:group.277 {
				uuid=4TDDarqTILvE+aYPfIoAAAEH;
				li:objects {
					ha:line.1 { x1=108000; y1=156000; x2=132000; y2=156000; stroke=wire; }
					ha:line.2 { x1=132000; y1=156000; x2=132000; y2=152000; stroke=wire; }
					ha:line.3 { x1=132000; y1=152000; x2=176000; y2=152000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.278 {
				li:conn {
					/2/277/1
					/2/263/2/1
				}
			}
			ha:connection.279 {
				li:conn {
					/2/277/3
					/2/34/5/1
				}
			}
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 1
     grid = 2.0480 mm
    }
   }
  }
}
