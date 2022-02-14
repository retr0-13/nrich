%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: nrich
Summary: Quickly enrich IPs with information about their open ports/ vulnerabilities/ software.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPLv3+
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://gitlab.com/shodan-public/nrich

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
