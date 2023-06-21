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
			ha:group.35 {
				uuid=MdqT+zstH2BY0fhajaEAAAK9;
				x=64000; y=0;
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
			ha:group.47 {
				uuid=MdqT+zstH2BY0fhajaEAAALJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=204000; y=180000;
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
					ha:text.2 { x1=110000; y1=124000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=CC2B
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.62 {
				uuid=MdqT+zstH2BY0fhajaEAAALg;
				x=56000; y=0;
				li:objects {
					ha:line.1 { x1=120000; y1=156000; x2=108000; y2=156000; stroke=wire; }
					ha:text.2 { x1=110000; y1=156000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=CC1A
					ha:role = { value=wire-net; prio=0; }
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
			ha:connection.80 {
				li:conn {
					/2/59/1
					/2/50/1/1
				}
			}
			ha:group.85 {
				uuid=MdqT+zstH2BY0fhajaEAAALi;
				li:objects {
					ha:line.4 { x1=108000; y1=112000; x2=176000; y2=112000; stroke=wire; }
					ha:text.5 { x1=166000; y1=112000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SBU2B
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.87 {
				uuid=MdqT+zstH2BY0fhajaEAAALj;
				li:objects {
					ha:line.1 { x1=176000; y1=148000; x2=108000; y2=148000; stroke=wire; }
					ha:text.2 { x1=166000; y1=148000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=DminusA
					ha:role = { value=wire-net; prio=0; }
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
					name=JMP0
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
					footprint=USB_A_PTH_Molex.rf
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
					footprint=USB_A_PTH_Molex.rf
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
					ha:text.4 { x1=166000; y1=144000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SBU1A
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
					ha:text.4 { x1=166000; y1=152000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=DplusA
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.278 {
				li:conn {
					/2/277/1
					/2/263/2/1
				}
			}
			ha:group.280 {
				uuid=gR6xY80BfSWB9yRoDXwAAABv; src_uuid=K6J/+nCrKSAGXQe5L5QAAABT;
				x=200000; y=132000; mirx=1;
				li:objects {
					ha:polygon.1 {
						li:outline {
							ha:line { x1=-20000; y1=-32000; x2=-20000; y2=36000; }
							ha:line { x1=-20000; y1=36000; x2=20000; y2=36000; }
							ha:line { x1=20000; y1=36000; x2=20000; y2=-32000; }
							ha:line { x1=20000; y1=-32000; x2=-20000; y2=-32000; }
						}
						stroke=sym-decor;
					}
					ha:group.2 {
						uuid=gR6xY80BfSWB9yRoDXwAAABw; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAB;
						x=20000; y=32000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAABx; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAC;
						x=20000; y=28000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAABy; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAD;
						x=20000; y=24000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAABz; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAE;
						x=20000; y=20000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB0; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAF;
						x=20000; y=16000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB1; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAG;
						x=20000; y=12000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB2; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAH;
						x=20000; y=8000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB3; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAI;
						x=20000; y=4000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB4; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAJ;
						x=20000; y=0;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB5; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAK;
						x=20000; y=-4000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB6; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAL;
						x=20000; y=-8000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB7; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAM;
						x=20000; y=-12000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB8; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAN;
						x=20000; y=-16000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB9; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAO;
						x=20000; y=-20000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB+; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAP;
						x=20000; y=-24000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAB/; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAQ;
						x=20000; y=-28000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAACA; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAR;
						x=-16000; y=36000; rot=90.000000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAACB; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAS;
						x=-12000; y=36000; rot=90.000000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAACC; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAT;
						x=-8000; y=36000; rot=90.000000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAACD; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAU;
						x=-4000; y=36000; rot=90.000000;
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
						uuid=gR6xY80BfSWB9yRoDXwAAACE; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAV;
						x=-16000; y=-32000; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=shldA
							pinnum=SGND2
							role=terminal
						}
					}
					ha:group.23 {
						uuid=gR6xY80BfSWB9yRoDXwAAACF; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAX;
						x=-12000; y=-32000; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=shldB
							pinnum=SGND1
							role=terminal
						}
					}
					ha:group.24 {
						uuid=gR6xY80BfSWB9yRoDXwAAACG; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAY;
						x=-8000; y=-32000; rot=-90.000000; mirx=1;
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
					ha:group.25 {
						uuid=gR6xY80BfSWB9yRoDXwAAACH; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAZ;
						x=-4000; y=-32000; rot=-90.000000; mirx=1;
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
					ha:group.26 {
						uuid=gR6xY80BfSWB9yRoDXwAAACI; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAa;
						x=0; y=-32000; rot=-90.000000; mirx=1;
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
					ha:group.27 {
						uuid=gR6xY80BfSWB9yRoDXwAAACJ; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAb;
						x=4000; y=-32000; rot=-90.000000; mirx=1;
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
					ha:text.28 { x1=12000; y1=36000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					footprint=USB_C_Breakout_M.rf
					name=USBC_M
					role=symbol
				}
			}
			ha:group.291 {
				uuid=gR6xY80BfSWB9yRoDXwAAACO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=196000; y=88000;
				li:objects {
					ha:group.1 {
						uuid=gR6xY80BfSWB9yRoDXwAAACP; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
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
			ha:connection.297 {
				li:conn {
					/2/47/1/1
					/2/35/8
				}
			}
			ha:connection.298 {
				li:conn {
					/2/280/4/1
					/2/62/1
				}
			}
			ha:connection.299 {
				li:conn {
					/2/280/5/1
					/2/277/3
				}
			}
			ha:connection.300 {
				li:conn {
					/2/280/6/1
					/2/87/1
				}
			}
			ha:connection.301 {
				li:conn {
					/2/280/7/1
					/2/275/2
				}
			}
			ha:connection.302 {
				li:conn {
					/2/280/12/1
					/2/59/1
				}
			}
			ha:connection.303 {
				li:conn {
					/2/280/15/1
					/2/85/4
				}
			}
			ha:connection.304 {
				li:conn {
					/2/280/18/1
					/2/35/2
				}
			}
			ha:connection.305 {
				li:conn {
					/2/280/19/1
					/2/35/3
				}
			}
			ha:connection.306 {
				li:conn {
					/2/280/20/1
					/2/35/5
				}
			}
			ha:connection.307 {
				li:conn {
					/2/280/21/1
					/2/35/8
				}
			}
			ha:group.317 {
				uuid=gR6xY80BfSWB9yRoDXwAAACU;
				li:objects {
					ha:line.1 { x1=208000; y1=92000; x2=208000; y2=96000; stroke=wire; }
					ha:line.4 { x1=200000; y1=92000; x2=200000; y2=92000; stroke=junction; }
					ha:line.5 { x1=196000; y1=92000; x2=208000; y2=92000; stroke=wire; }
					ha:line.7 { x1=204000; y1=92000; x2=204000; y2=96000; stroke=wire; }
					ha:line.8 { x1=196000; y1=88000; x2=196000; y2=96000; stroke=wire; }
					ha:line.9 { x1=204000; y1=92000; x2=204000; y2=92000; stroke=junction; }
					ha:line.10 { x1=196000; y1=92000; x2=196000; y2=92000; stroke=junction; }
					ha:line.11 { x1=200000; y1=92000; x2=200000; y2=96000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.318 {
				li:conn {
					/2/317/1
					/2/280/24/1
				}
			}
			ha:connection.319 {
				li:conn {
					/2/317/7
					/2/280/25/1
				}
			}
			ha:connection.320 {
				li:conn {
					/2/317/8
					/2/280/27/1
				}
			}
			ha:connection.321 {
				li:conn {
					/2/317/8
					/2/291/1/1
				}
			}
			ha:connection.322 {
				li:conn {
					/2/317/11
					/2/280/26/1
				}
			}
			ha:group.326 {
				uuid=gR6xY80BfSWB9yRoDXwAAAC4; src_uuid=gR6xY80BfSWB9yRoDXwAAAC1;
				x=224000; y=88000;
				li:objects {
					ha:text.1 { x1=0; y1=6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=gR6xY80BfSWB9yRoDXwAAAC5; src_uuid=gR6xY80BfSWB9yRoDXwAAAC2;
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
						uuid=gR6xY80BfSWB9yRoDXwAAAC6; src_uuid=gR6xY80BfSWB9yRoDXwAAAC3;
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
					name=SGNDM
					role=symbol
				}
			}
			ha:group.329 {
				uuid=gR6xY80BfSWB9yRoDXwAAAC9; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=216000; y=88000;
				li:objects {
					ha:group.1 {
						uuid=gR6xY80BfSWB9yRoDXwAAAC+; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
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
			ha:group.331 {
				uuid=gR6xY80BfSWB9yRoDXwAAAC/;
				li:objects {
					ha:line.1 { x1=220000; y1=88000; x2=216000; y2=88000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.332 {
				li:conn {
					/2/331/1
					/2/326/2/1
				}
			}
			ha:connection.336 {
				li:conn {
					/2/331/1
					/2/329/1/1
				}
			}
			ha:group.338 {
				uuid=gR6xY80BfSWB9yRoDXwAAADB;
				li:objects {
					ha:line.1 { x1=216000; y1=92000; x2=216000; y2=96000; stroke=wire; }
					ha:line.2 { x1=216000; y1=92000; x2=216000; y2=92000; stroke=junction; }
					ha:line.3 { x1=212000; y1=96000; x2=212000; y2=92000; stroke=wire; }
					ha:line.4 { x1=212000; y1=92000; x2=220000; y2=92000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.339 {
				li:conn {
					/2/338/1
					/2/280/22/1
				}
			}
			ha:connection.340 {
				li:conn {
					/2/338/3
					/2/280/23/1
				}
			}
			ha:connection.341 {
				li:conn {
					/2/338/4
					/2/326/3/1
				}
			}
			ha:group.342 {
				uuid=kZANytQbOJsJFfgmBO0AAACF; src_uuid=MdqT+zstH2BY0fhajaEAAAK9;
				x=132000; y=0;
				li:objects {
					ha:line.1 { x1=140000; y1=176000; x2=152000; y2=176000; stroke=wire; }
					ha:line.2 { x1=152000; y1=176000; x2=152000; y2=172000; stroke=wire; }
					ha:line.3 { x1=148000; y1=172000; x2=148000; y2=176000; stroke=wire; }
					ha:line.5 { x1=144000; y1=176000; x2=144000; y2=172000; stroke=wire; }
					ha:line.8 { x1=140000; y1=172000; x2=140000; y2=180000; stroke=wire; }
					ha:line.13 { x1=144000; y1=176000; x2=144000; y2=176000; stroke=junction; }
					ha:line.14 { x1=140000; y1=176000; x2=140000; y2=176000; stroke=junction; }
					ha:line.15 { x1=148000; y1=176000; x2=148000; y2=176000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.343 {
				uuid=kZANytQbOJsJFfgmBO0AAACG; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=272000; y=180000;
				li:objects {
					ha:group.1 {
						uuid=kZANytQbOJsJFfgmBO0AAACH; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
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
			ha:group.345 {
				uuid=kZANytQbOJsJFfgmBO0AAACI; src_uuid=K6J/+nCrKSAGXQe5L5QAAABT;
				x=268000; y=132000; mirx=1;
				li:objects {
					ha:polygon.1 {
						li:outline {
							ha:line { x1=-20000; y1=-32000; x2=-20000; y2=36000; }
							ha:line { x1=-20000; y1=36000; x2=20000; y2=36000; }
							ha:line { x1=20000; y1=36000; x2=20000; y2=-32000; }
							ha:line { x1=20000; y1=-32000; x2=-20000; y2=-32000; }
						}
						stroke=sym-decor;
					}
					ha:group.2 {
						uuid=kZANytQbOJsJFfgmBO0AAACJ; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAB;
						x=20000; y=32000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACK; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAC;
						x=20000; y=28000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACL; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAD;
						x=20000; y=24000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACM; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAE;
						x=20000; y=20000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACN; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAF;
						x=20000; y=16000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACO; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAG;
						x=20000; y=12000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACP; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAH;
						x=20000; y=8000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACQ; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAI;
						x=20000; y=4000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACR; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAJ;
						x=20000; y=0;
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
						uuid=kZANytQbOJsJFfgmBO0AAACS; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAK;
						x=20000; y=-4000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACT; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAL;
						x=20000; y=-8000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACU; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAM;
						x=20000; y=-12000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACV; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAN;
						x=20000; y=-16000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACW; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAO;
						x=20000; y=-20000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACX; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAP;
						x=20000; y=-24000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACY; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAQ;
						x=20000; y=-28000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACZ; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAR;
						x=-16000; y=36000; rot=90.000000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACa; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAS;
						x=-12000; y=36000; rot=90.000000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACb; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAT;
						x=-8000; y=36000; rot=90.000000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACc; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAU;
						x=-4000; y=36000; rot=90.000000;
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
						uuid=kZANytQbOJsJFfgmBO0AAACd; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAV;
						x=-16000; y=-32000; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=shldA
							pinnum=SGND2
							role=terminal
						}
					}
					ha:group.23 {
						uuid=kZANytQbOJsJFfgmBO0AAACe; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAX;
						x=-12000; y=-32000; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=shldB
							pinnum=SGND1
							role=terminal
						}
					}
					ha:group.24 {
						uuid=kZANytQbOJsJFfgmBO0AAACf; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAY;
						x=-8000; y=-32000; rot=-90.000000; mirx=1;
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
					ha:group.25 {
						uuid=kZANytQbOJsJFfgmBO0AAACg; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAZ;
						x=-4000; y=-32000; rot=-90.000000; mirx=1;
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
					ha:group.26 {
						uuid=kZANytQbOJsJFfgmBO0AAACh; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAa;
						x=0; y=-32000; rot=-90.000000; mirx=1;
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
					ha:group.27 {
						uuid=kZANytQbOJsJFfgmBO0AAACi; src_uuid=sIb9RtXRmDXN7pA5f1oAAAAb;
						x=4000; y=-32000; rot=-90.000000; mirx=1;
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
					ha:text.28 { x1=12000; y1=36000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					footprint=USB_C_Breakout_F.rf
					name=USBC_F
					role=symbol
				}
			}
			ha:group.350 {
				uuid=kZANytQbOJsJFfgmBO0AAACj; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=264000; y=88000;
				li:objects {
					ha:group.1 {
						uuid=kZANytQbOJsJFfgmBO0AAACk; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
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
			ha:group.351 {
				uuid=kZANytQbOJsJFfgmBO0AAACl; src_uuid=gR6xY80BfSWB9yRoDXwAAACU;
				x=68000; y=0;
				li:objects {
					ha:line.1 { x1=208000; y1=92000; x2=208000; y2=96000; stroke=wire; }
					ha:line.5 { x1=196000; y1=92000; x2=208000; y2=92000; stroke=wire; }
					ha:line.7 { x1=204000; y1=92000; x2=204000; y2=96000; stroke=wire; }
					ha:line.8 { x1=196000; y1=88000; x2=196000; y2=96000; stroke=wire; }
					ha:line.11 { x1=200000; y1=92000; x2=200000; y2=96000; stroke=wire; }
					ha:line.15 { x1=196000; y1=92000; x2=196000; y2=92000; stroke=junction; }
					ha:line.16 { x1=200000; y1=92000; x2=200000; y2=92000; stroke=junction; }
					ha:line.17 { x1=204000; y1=92000; x2=204000; y2=92000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.357 {
				uuid=kZANytQbOJsJFfgmBO0AAACm; src_uuid=gR6xY80BfSWB9yRoDXwAAAC1;
				x=292000; y=88000;
				li:objects {
					ha:text.1 { x1=0; y1=6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=kZANytQbOJsJFfgmBO0AAACn; src_uuid=gR6xY80BfSWB9yRoDXwAAAC2;
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
						uuid=kZANytQbOJsJFfgmBO0AAACo; src_uuid=gR6xY80BfSWB9yRoDXwAAAC3;
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
					name=SGNDF
					role=symbol
				}
			}
			ha:group.358 {
				uuid=kZANytQbOJsJFfgmBO0AAACp; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=284000; y=88000;
				li:objects {
					ha:group.1 {
						uuid=kZANytQbOJsJFfgmBO0AAACq; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
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
			ha:group.359 {
				uuid=kZANytQbOJsJFfgmBO0AAACr; src_uuid=gR6xY80BfSWB9yRoDXwAAAC/;
				x=68000; y=0;
				li:objects {
					ha:line.1 { x1=220000; y1=88000; x2=216000; y2=88000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.362 {
				uuid=kZANytQbOJsJFfgmBO0AAACs; src_uuid=gR6xY80BfSWB9yRoDXwAAADB;
				x=68000; y=0;
				li:objects {
					ha:line.1 { x1=216000; y1=92000; x2=216000; y2=96000; stroke=wire; }
					ha:line.3 { x1=212000; y1=96000; x2=212000; y2=92000; stroke=wire; }
					ha:line.4 { x1=212000; y1=92000; x2=220000; y2=92000; stroke=wire; }
					ha:line.6 { x1=216000; y1=92000; x2=216000; y2=92000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.366 {
				li:conn {
					/2/343/1/1
					/2/342/8
				}
			}
			ha:connection.367 {
				li:conn {
					/2/345/18/1
					/2/342/2
				}
			}
			ha:connection.368 {
				li:conn {
					/2/345/19/1
					/2/342/3
				}
			}
			ha:connection.369 {
				li:conn {
					/2/345/20/1
					/2/342/5
				}
			}
			ha:connection.370 {
				li:conn {
					/2/345/21/1
					/2/342/8
				}
			}
			ha:connection.371 {
				li:conn {
					/2/351/1
					/2/345/24/1
				}
			}
			ha:connection.372 {
				li:conn {
					/2/351/7
					/2/345/25/1
				}
			}
			ha:connection.373 {
				li:conn {
					/2/351/8
					/2/350/1/1
				}
			}
			ha:connection.374 {
				li:conn {
					/2/351/8
					/2/345/27/1
				}
			}
			ha:connection.375 {
				li:conn {
					/2/351/11
					/2/345/26/1
				}
			}
			ha:connection.376 {
				li:conn {
					/2/359/1
					/2/357/2/1
				}
			}
			ha:connection.377 {
				li:conn {
					/2/359/1
					/2/358/1/1
				}
			}
			ha:connection.378 {
				li:conn {
					/2/362/1
					/2/345/22/1
				}
			}
			ha:connection.379 {
				li:conn {
					/2/362/3
					/2/345/23/1
				}
			}
			ha:connection.380 {
				li:conn {
					/2/362/4
					/2/357/3/1
				}
			}
			ha:group.382 {
				uuid=kZANytQbOJsJFfgmBO0AAACt;
				li:objects {
					ha:line.1 { x1=234000; y1=156000; x2=244000; y2=156000; stroke=wire; }
					ha:text.4 { x1=234000; y1=156000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=CC1A
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.383 {
				li:conn {
					/2/382/1
					/2/345/4/1
				}
			}
			ha:group.384 {
				uuid=kZANytQbOJsJFfgmBO0AAACv; src_uuid=kZANytQbOJsJFfgmBO0AAACu;
				x=240000; y=152000;
				li:objects {
					ha:line.1 { x1=-6000; y1=0; x2=4000; y2=0; stroke=wire; }
					ha:text.2 { x1=-6000; y1=0; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=DplusA
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.385 {
				li:conn {
					/2/384/1
					/2/345/5/1
				}
			}
			ha:group.386 {
				uuid=kZANytQbOJsJFfgmBO0AAACw; src_uuid=kZANytQbOJsJFfgmBO0AAACu;
				x=240000; y=148000;
				li:objects {
					ha:line.1 { x1=-6000; y1=0; x2=4000; y2=0; stroke=wire; }
					ha:text.2 { x1=-6000; y1=0; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=DminusA
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.387 {
				li:conn {
					/2/386/1
					/2/345/6/1
				}
			}
			ha:group.388 {
				uuid=kZANytQbOJsJFfgmBO0AAACx; src_uuid=kZANytQbOJsJFfgmBO0AAACu;
				x=240000; y=144000;
				li:objects {
					ha:line.1 { x1=-6000; y1=0; x2=4000; y2=0; stroke=wire; }
					ha:text.2 { x1=-6000; y1=0; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SBU1A
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.389 {
				li:conn {
					/2/388/1
					/2/345/7/1
				}
			}
			ha:group.392 {
				uuid=kZANytQbOJsJFfgmBO0AAACz; src_uuid=kZANytQbOJsJFfgmBO0AAACu;
				x=240000; y=112000;
				li:objects {
					ha:line.1 { x1=-6000; y1=0; x2=4000; y2=0; stroke=wire; }
					ha:text.2 { x1=-6000; y1=0; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SBU2B
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.393 {
				li:conn {
					/2/392/1
					/2/345/15/1
				}
			}
			ha:group.394 {
				uuid=kZANytQbOJsJFfgmBO0AAAC0; src_uuid=kZANytQbOJsJFfgmBO0AAACu;
				x=240000; y=124000;
				li:objects {
					ha:line.1 { x1=-6000; y1=0; x2=4000; y2=0; stroke=wire; }
					ha:text.2 { x1=-6000; y1=0; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=CC2B
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.395 {
				li:conn {
					/2/394/1
					/2/345/12/1
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
     grids_idx = 0
     grid = 1.0240 mm
    }
   }
  }
}
