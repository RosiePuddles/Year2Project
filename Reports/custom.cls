\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{custom}[2014/08/16 Custom document class for this project]

\LoadClass[11pt]{article}
\DeclareOption{coverpage}{
\renewcommand\maketitle{
\begin{titlepage}\begin{center}
	\vspace*{5cm}
	\huge\@title
	\vspace{2cm}
	\begin{tabular}[t]{c}
		\Large\@author
	\end{tabular}
	\vspace{1cm}
	\@date
\end{center}\end{titlepage}
}
}

\ProcessOptions\relax
\usepackage[a4paper,total={155mm,250mm}]{geometry}
\RequirePackage{authblk}
\RequirePackage{titlesec}
\setlength\parskip{0.5em}
\usepackage{indentfirst}
\usepackage{hyperref}
\usepackage{cleveref}
\author{Simon Zaragoza-Dorwald, Rosie Bartlett, Lavish Bhojani, Dravin Gupta, Joseph Dunne, Callum Gray}
\newcommand{\Title}[1]{\title{Creating a Virtual Reality Meditation Visualisation System\\\LARGE{#1}}\vspace{1cm}}
\titlespacing\section{0pt}{10pt}{-3pt}
\titlespacing\subsection{0pt}{5pt}{-3pt}
\titlespacing\subsubsection{0pt}{5pt}{-3pt}
\renewcommand{\baselinestretch}{1.5}
\renewcommand\@maketitle{
	\newpage
	\null
	\vskip 2em
	\begin{center}
		\huge\@title
		\vskip 2.5em
		\large\lineskip .5em
		\begin{tabular}[t]{c}
			\@author
		\end{tabular}\par
		\vskip 0.5em
		\large \@date
	\end{center}
	\par
  \vskip 1.5em
}
\renewcommand\theequation{\thesection.\arabic{equation}}
\renewcommand\thefigure{\thesection.\arabic{figure}}
\usepackage{etoolbox}
\preto{\section}{\setcounter{equation}{0}\setcounter{figure}{0}}
\preto{\appendix}{\newpage}
\newcommand{\finish}{
\listoffigures
\listoftables
}
