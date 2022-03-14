impl serde::Serialize for Container {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dockerfile.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.Container", len)?;
        if !self.dockerfile.is_empty() {
            struct_ser.serialize_field("dockerfile", &self.dockerfile)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Container {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dockerfile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Dockerfile,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dockerfile" => Ok(GeneratedField::Dockerfile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Container;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.Container")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Container, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dockerfile = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Dockerfile => {
                            if dockerfile.is_some() {
                                return Err(serde::de::Error::duplicate_field("dockerfile"));
                            }
                            dockerfile = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Container {
                    dockerfile: dockerfile.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.Container", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DelResourceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.revision.is_empty() {
            len += 1;
        }
        if !self.namespace.is_empty() {
            len += 1;
        }
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.DelResourceRequest", len)?;
        if !self.revision.is_empty() {
            struct_ser.serialize_field("revision", &self.revision)?;
        }
        if !self.namespace.is_empty() {
            struct_ser.serialize_field("namespace", &self.namespace)?;
        }
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DelResourceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "revision",
            "namespace",
            "kind",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Revision,
            Namespace,
            Kind,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "revision" => Ok(GeneratedField::Revision),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "kind" => Ok(GeneratedField::Kind),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DelResourceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.DelResourceRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DelResourceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut revision = None;
                let mut namespace = None;
                let mut kind = None;
                let mut name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Revision => {
                            if revision.is_some() {
                                return Err(serde::de::Error::duplicate_field("revision"));
                            }
                            revision = Some(map.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace = Some(map.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DelResourceRequest {
                    revision: revision.unwrap_or_default(),
                    namespace: namespace.unwrap_or_default(),
                    kind: kind.unwrap_or_default(),
                    name: name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.DelResourceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DelResourceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.updates != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.DelResourceResponse", len)?;
        if self.updates != 0 {
            struct_ser.serialize_field("updates", &self.updates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DelResourceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "updates",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Updates,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "updates" => Ok(GeneratedField::Updates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DelResourceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.DelResourceResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DelResourceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut updates = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Updates => {
                            if updates.is_some() {
                                return Err(serde::de::Error::duplicate_field("updates"));
                            }
                            updates = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(DelResourceResponse {
                    updates: updates.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.DelResourceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DelResourcesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.revision.is_empty() {
            len += 1;
        }
        if !self.namespaces.is_empty() {
            len += 1;
        }
        if !self.kinds.is_empty() {
            len += 1;
        }
        if !self.bql.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.DelResourcesRequest", len)?;
        if !self.revision.is_empty() {
            struct_ser.serialize_field("revision", &self.revision)?;
        }
        if !self.namespaces.is_empty() {
            struct_ser.serialize_field("namespaces", &self.namespaces)?;
        }
        if !self.kinds.is_empty() {
            struct_ser.serialize_field("kinds", &self.kinds)?;
        }
        if !self.bql.is_empty() {
            struct_ser.serialize_field("bql", &self.bql)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DelResourcesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "revision",
            "namespaces",
            "kinds",
            "bql",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Revision,
            Namespaces,
            Kinds,
            Bql,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "revision" => Ok(GeneratedField::Revision),
                            "namespaces" => Ok(GeneratedField::Namespaces),
                            "kinds" => Ok(GeneratedField::Kinds),
                            "bql" => Ok(GeneratedField::Bql),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DelResourcesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.DelResourcesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DelResourcesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut revision = None;
                let mut namespaces = None;
                let mut kinds = None;
                let mut bql = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Revision => {
                            if revision.is_some() {
                                return Err(serde::de::Error::duplicate_field("revision"));
                            }
                            revision = Some(map.next_value()?);
                        }
                        GeneratedField::Namespaces => {
                            if namespaces.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaces"));
                            }
                            namespaces = Some(map.next_value()?);
                        }
                        GeneratedField::Kinds => {
                            if kinds.is_some() {
                                return Err(serde::de::Error::duplicate_field("kinds"));
                            }
                            kinds = Some(map.next_value()?);
                        }
                        GeneratedField::Bql => {
                            if bql.is_some() {
                                return Err(serde::de::Error::duplicate_field("bql"));
                            }
                            bql = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DelResourcesRequest {
                    revision: revision.unwrap_or_default(),
                    namespaces: namespaces.unwrap_or_default(),
                    kinds: kinds.unwrap_or_default(),
                    bql: bql.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.DelResourcesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Function {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.inputs.is_empty() {
            len += 1;
        }
        if !self.outputs.is_empty() {
            len += 1;
        }
        if !self.base_command.is_empty() {
            len += 1;
        }
        if !self.runtime_capabilities.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.Function", len)?;
        if !self.inputs.is_empty() {
            struct_ser.serialize_field("inputs", &self.inputs)?;
        }
        if !self.outputs.is_empty() {
            struct_ser.serialize_field("outputs", &self.outputs)?;
        }
        if !self.base_command.is_empty() {
            struct_ser.serialize_field("baseCommand", &self.base_command)?;
        }
        if !self.runtime_capabilities.is_empty() {
            struct_ser.serialize_field("runtimeCapabilities", &self.runtime_capabilities)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Function {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inputs",
            "outputs",
            "baseCommand",
            "runtimeCapabilities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Inputs,
            Outputs,
            BaseCommand,
            RuntimeCapabilities,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "inputs" => Ok(GeneratedField::Inputs),
                            "outputs" => Ok(GeneratedField::Outputs),
                            "baseCommand" => Ok(GeneratedField::BaseCommand),
                            "runtimeCapabilities" => Ok(GeneratedField::RuntimeCapabilities),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Function;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.Function")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Function, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inputs = None;
                let mut outputs = None;
                let mut base_command = None;
                let mut runtime_capabilities = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Inputs => {
                            if inputs.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputs"));
                            }
                            inputs = Some(map.next_value()?);
                        }
                        GeneratedField::Outputs => {
                            if outputs.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputs"));
                            }
                            outputs = Some(map.next_value()?);
                        }
                        GeneratedField::BaseCommand => {
                            if base_command.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseCommand"));
                            }
                            base_command = Some(map.next_value()?);
                        }
                        GeneratedField::RuntimeCapabilities => {
                            if runtime_capabilities.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeCapabilities"));
                            }
                            runtime_capabilities = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Function {
                    inputs: inputs.unwrap_or_default(),
                    outputs: outputs.unwrap_or_default(),
                    base_command: base_command.unwrap_or_default(),
                    runtime_capabilities: runtime_capabilities.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.Function", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKindsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ph {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.GetKindsRequest", len)?;
        if self.ph {
            struct_ser.serialize_field("ph", &self.ph)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKindsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ph",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ph,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ph" => Ok(GeneratedField::Ph),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetKindsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.GetKindsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetKindsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ph = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ph => {
                            if ph.is_some() {
                                return Err(serde::de::Error::duplicate_field("ph"));
                            }
                            ph = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetKindsRequest {
                    ph: ph.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.GetKindsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKindsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kinds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.GetKindsResponse", len)?;
        if !self.kinds.is_empty() {
            struct_ser.serialize_field("kinds", &self.kinds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKindsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kinds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kinds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kinds" => Ok(GeneratedField::Kinds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetKindsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.GetKindsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetKindsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kinds = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Kinds => {
                            if kinds.is_some() {
                                return Err(serde::de::Error::duplicate_field("kinds"));
                            }
                            kinds = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetKindsResponse {
                    kinds: kinds.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.GetKindsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetNamespacesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ph {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.GetNamespacesRequest", len)?;
        if self.ph {
            struct_ser.serialize_field("ph", &self.ph)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetNamespacesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ph",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ph,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ph" => Ok(GeneratedField::Ph),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetNamespacesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.GetNamespacesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetNamespacesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ph = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ph => {
                            if ph.is_some() {
                                return Err(serde::de::Error::duplicate_field("ph"));
                            }
                            ph = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetNamespacesRequest {
                    ph: ph.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.GetNamespacesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetNamespacesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.namespaces.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.GetNamespacesResponse", len)?;
        if !self.namespaces.is_empty() {
            struct_ser.serialize_field("namespaces", &self.namespaces)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetNamespacesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespaces",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Namespaces,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "namespaces" => Ok(GeneratedField::Namespaces),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetNamespacesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.GetNamespacesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetNamespacesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespaces = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Namespaces => {
                            if namespaces.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaces"));
                            }
                            namespaces = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetNamespacesResponse {
                    namespaces: namespaces.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.GetNamespacesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.revision.is_empty() {
            len += 1;
        }
        if !self.namespace.is_empty() {
            len += 1;
        }
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.GetResourceRequest", len)?;
        if !self.revision.is_empty() {
            struct_ser.serialize_field("revision", &self.revision)?;
        }
        if !self.namespace.is_empty() {
            struct_ser.serialize_field("namespace", &self.namespace)?;
        }
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "revision",
            "namespace",
            "kind",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Revision,
            Namespace,
            Kind,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "revision" => Ok(GeneratedField::Revision),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "kind" => Ok(GeneratedField::Kind),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.GetResourceRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetResourceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut revision = None;
                let mut namespace = None;
                let mut kind = None;
                let mut name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Revision => {
                            if revision.is_some() {
                                return Err(serde::de::Error::duplicate_field("revision"));
                            }
                            revision = Some(map.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace = Some(map.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetResourceRequest {
                    revision: revision.unwrap_or_default(),
                    namespace: namespace.unwrap_or_default(),
                    kind: kind.unwrap_or_default(),
                    name: name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.GetResourceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourcesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.revision.is_empty() {
            len += 1;
        }
        if !self.namespaces.is_empty() {
            len += 1;
        }
        if !self.kinds.is_empty() {
            len += 1;
        }
        if !self.bql.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.GetResourcesRequest", len)?;
        if !self.revision.is_empty() {
            struct_ser.serialize_field("revision", &self.revision)?;
        }
        if !self.namespaces.is_empty() {
            struct_ser.serialize_field("namespaces", &self.namespaces)?;
        }
        if !self.kinds.is_empty() {
            struct_ser.serialize_field("kinds", &self.kinds)?;
        }
        if !self.bql.is_empty() {
            struct_ser.serialize_field("bql", &self.bql)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourcesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "revision",
            "namespaces",
            "kinds",
            "bql",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Revision,
            Namespaces,
            Kinds,
            Bql,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "revision" => Ok(GeneratedField::Revision),
                            "namespaces" => Ok(GeneratedField::Namespaces),
                            "kinds" => Ok(GeneratedField::Kinds),
                            "bql" => Ok(GeneratedField::Bql),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourcesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.GetResourcesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetResourcesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut revision = None;
                let mut namespaces = None;
                let mut kinds = None;
                let mut bql = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Revision => {
                            if revision.is_some() {
                                return Err(serde::de::Error::duplicate_field("revision"));
                            }
                            revision = Some(map.next_value()?);
                        }
                        GeneratedField::Namespaces => {
                            if namespaces.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaces"));
                            }
                            namespaces = Some(map.next_value()?);
                        }
                        GeneratedField::Kinds => {
                            if kinds.is_some() {
                                return Err(serde::de::Error::duplicate_field("kinds"));
                            }
                            kinds = Some(map.next_value()?);
                        }
                        GeneratedField::Bql => {
                            if bql.is_some() {
                                return Err(serde::de::Error::duplicate_field("bql"));
                            }
                            bql = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetResourcesRequest {
                    revision: revision.unwrap_or_default(),
                    namespaces: namespaces.unwrap_or_default(),
                    kinds: kinds.unwrap_or_default(),
                    bql: bql.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.GetResourcesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourcesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resources.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.GetResourcesResponse", len)?;
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourcesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resources,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resources" => Ok(GeneratedField::Resources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourcesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.GetResourcesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetResourcesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resources = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Resources => {
                            if resources.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetResourcesResponse {
                    resources: resources.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.GetResourcesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRevisionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ph {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.GetRevisionsRequest", len)?;
        if self.ph {
            struct_ser.serialize_field("ph", &self.ph)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRevisionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ph",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ph,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ph" => Ok(GeneratedField::Ph),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRevisionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.GetRevisionsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetRevisionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ph = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ph => {
                            if ph.is_some() {
                                return Err(serde::de::Error::duplicate_field("ph"));
                            }
                            ph = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetRevisionsRequest {
                    ph: ph.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.GetRevisionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRevisionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.revisions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.GetRevisionsResponse", len)?;
        if !self.revisions.is_empty() {
            struct_ser.serialize_field("revisions", &self.revisions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRevisionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "revisions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Revisions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "revisions" => Ok(GeneratedField::Revisions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRevisionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.GetRevisionsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetRevisionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut revisions = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Revisions => {
                            if revisions.is_some() {
                                return Err(serde::de::Error::duplicate_field("revisions"));
                            }
                            revisions = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetRevisionsResponse {
                    revisions: revisions.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.GetRevisionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Parameter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        if self.default_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.Parameter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.kind != 0 {
            let v = ParameterKind::from_i32(self.kind)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if let Some(v) = self.default_value.as_ref() {
            struct_ser.serialize_field("defaultValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Parameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "kind",
            "defaultValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Kind,
            DefaultValue,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "kind" => Ok(GeneratedField::Kind),
                            "defaultValue" => Ok(GeneratedField::DefaultValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Parameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.Parameter")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Parameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                let mut description = None;
                let mut kind = None;
                let mut default_value = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind = Some(map.next_value::<ParameterKind>()? as i32);
                        }
                        GeneratedField::DefaultValue => {
                            if default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Parameter {
                    name: name.unwrap_or_default(),
                    description: description.unwrap_or_default(),
                    kind: kind.unwrap_or_default(),
                    default_value,
                })
            }
        }
        deserializer.deserialize_struct("bda.Parameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParameterKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Integer => "INTEGER",
            Self::Real => "REAL",
            Self::Boolean => "BOOLEAN",
            Self::Text => "TEXT",
            Self::Json => "JSON",
            Self::Url => "URL",
            Self::Path => "PATH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ParameterKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INTEGER",
            "REAL",
            "BOOLEAN",
            "TEXT",
            "JSON",
            "URL",
            "PATH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParameterKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ParameterKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ParameterKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "INTEGER" => Ok(ParameterKind::Integer),
                    "REAL" => Ok(ParameterKind::Real),
                    "BOOLEAN" => Ok(ParameterKind::Boolean),
                    "TEXT" => Ok(ParameterKind::Text),
                    "JSON" => Ok(ParameterKind::Json),
                    "URL" => Ok(ParameterKind::Url),
                    "PATH" => Ok(ParameterKind::Path),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PutResourceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.PutResourceRequest", len)?;
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutResourceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resource,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resource" => Ok(GeneratedField::Resource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutResourceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.PutResourceRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PutResourceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Resource => {
                            if resource.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PutResourceRequest {
                    resource,
                })
            }
        }
        deserializer.deserialize_struct("bda.PutResourceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PutResourceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.updates != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.PutResourceResponse", len)?;
        if self.updates != 0 {
            struct_ser.serialize_field("updates", &self.updates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PutResourceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "updates",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Updates,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "updates" => Ok(GeneratedField::Updates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PutResourceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.PutResourceResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PutResourceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut updates = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Updates => {
                            if updates.is_some() {
                                return Err(serde::de::Error::duplicate_field("updates"));
                            }
                            updates = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(PutResourceResponse {
                    updates: updates.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("bda.PutResourceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Resource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.revision.is_empty() {
            len += 1;
        }
        if !self.namespace.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.resource_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.Resource", len)?;
        if !self.revision.is_empty() {
            struct_ser.serialize_field("revision", &self.revision)?;
        }
        if !self.namespace.is_empty() {
            struct_ser.serialize_field("namespace", &self.namespace)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if let Some(v) = self.resource_kind.as_ref() {
            match v {
                resource::ResourceKind::Function(v) => {
                    struct_ser.serialize_field("function", v)?;
                }
                resource::ResourceKind::Runtime(v) => {
                    struct_ser.serialize_field("runtime", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Resource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "revision",
            "namespace",
            "name",
            "description",
            "tags",
            "function",
            "runtime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Revision,
            Namespace,
            Name,
            Description,
            Tags,
            Function,
            Runtime,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "revision" => Ok(GeneratedField::Revision),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "tags" => Ok(GeneratedField::Tags),
                            "function" => Ok(GeneratedField::Function),
                            "runtime" => Ok(GeneratedField::Runtime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Resource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.Resource")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Resource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut revision = None;
                let mut namespace = None;
                let mut name = None;
                let mut description = None;
                let mut tags = None;
                let mut resource_kind = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Revision => {
                            if revision.is_some() {
                                return Err(serde::de::Error::duplicate_field("revision"));
                            }
                            revision = Some(map.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        GeneratedField::Tags => {
                            if tags.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags = Some(map.next_value()?);
                        }
                        GeneratedField::Function => {
                            if resource_kind.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            resource_kind = Some(resource::ResourceKind::Function(map.next_value()?));
                        }
                        GeneratedField::Runtime => {
                            if resource_kind.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtime"));
                            }
                            resource_kind = Some(resource::ResourceKind::Runtime(map.next_value()?));
                        }
                    }
                }
                Ok(Resource {
                    revision: revision.unwrap_or_default(),
                    namespace: namespace.unwrap_or_default(),
                    name: name.unwrap_or_default(),
                    description: description.unwrap_or_default(),
                    tags: tags.unwrap_or_default(),
                    resource_kind,
                })
            }
        }
        deserializer.deserialize_struct("bda.Resource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Runtime {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.capabilities.is_empty() {
            len += 1;
        }
        if self.runtime_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("bda.Runtime", len)?;
        if !self.capabilities.is_empty() {
            struct_ser.serialize_field("capabilities", &self.capabilities)?;
        }
        if let Some(v) = self.runtime_kind.as_ref() {
            match v {
                runtime::RuntimeKind::Container(v) => {
                    struct_ser.serialize_field("container", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Runtime {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "capabilities",
            "container",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Capabilities,
            Container,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "capabilities" => Ok(GeneratedField::Capabilities),
                            "container" => Ok(GeneratedField::Container),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Runtime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct bda.Runtime")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Runtime, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut capabilities = None;
                let mut runtime_kind = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Capabilities => {
                            if capabilities.is_some() {
                                return Err(serde::de::Error::duplicate_field("capabilities"));
                            }
                            capabilities = Some(map.next_value()?);
                        }
                        GeneratedField::Container => {
                            if runtime_kind.is_some() {
                                return Err(serde::de::Error::duplicate_field("container"));
                            }
                            runtime_kind = Some(runtime::RuntimeKind::Container(map.next_value()?));
                        }
                    }
                }
                Ok(Runtime {
                    capabilities: capabilities.unwrap_or_default(),
                    runtime_kind,
                })
            }
        }
        deserializer.deserialize_struct("bda.Runtime", FIELDS, GeneratedVisitor)
    }
}
